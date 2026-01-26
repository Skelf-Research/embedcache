//! LLM client abstraction for chunking

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// LLM provider type
#[derive(Debug, Clone, PartialEq)]
pub enum LLMProvider {
    Ollama,
    OpenAI,
    Anthropic,
}

impl LLMProvider {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ollama" => Some(Self::Ollama),
            "openai" => Some(Self::OpenAI),
            "anthropic" => Some(Self::Anthropic),
            _ => None,
        }
    }
}

/// Configuration for LLM-based chunking
#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub provider: LLMProvider,
    pub model: String,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub timeout_secs: u64,
}

impl LLMConfig {
    /// Create LLMConfig from server config if LLM is configured
    pub fn from_server_config(config: &crate::config::ServerConfig) -> Option<Self> {
        let provider_str = config.llm_provider.as_ref()?;
        let provider = LLMProvider::from_str(provider_str)?;

        Some(Self {
            provider,
            model: config
                .llm_model
                .clone()
                .unwrap_or_else(|| "llama3".to_string()),
            base_url: config.llm_base_url.clone(),
            api_key: config.llm_api_key.clone(),
            timeout_secs: config.llm_timeout,
        })
    }
}

/// Trait for LLM client implementations
#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String>;
}

/// HTTP-based LLM client for Ollama
pub struct OllamaClient {
    base_url: String,
    model: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(config: &LLMConfig) -> Self {
        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| "http://localhost:11434".to_string());

        Self {
            base_url,
            model: config.model.clone(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(config.timeout_secs))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[async_trait]
impl LLMClient for OllamaClient {
    async fn complete(&self, prompt: &str) -> Result<String> {
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?;

        let ollama_response: OllamaResponse = response.json().await?;
        Ok(ollama_response.response)
    }
}

/// HTTP-based LLM client for OpenAI-compatible APIs
pub struct OpenAIClient {
    base_url: String,
    model: String,
    api_key: String,
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(config: &LLMConfig) -> Result<Self> {
        let api_key = config
            .api_key
            .clone()
            .ok_or_else(|| anyhow::anyhow!("OpenAI API key is required"))?;

        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string());

        Ok(Self {
            base_url,
            model: config.model.clone(),
            api_key,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(config.timeout_secs))
                .build()?,
        })
    }
}

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessageResponse,
}

#[derive(Deserialize)]
struct OpenAIMessageResponse {
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn complete(&self, prompt: &str) -> Result<String> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![OpenAIMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let openai_response: OpenAIResponse = response.json().await?;
        openai_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))
    }
}

/// Create an LLM client based on configuration
pub fn create_llm_client(config: &LLMConfig) -> Result<Box<dyn LLMClient>> {
    match config.provider {
        LLMProvider::Ollama => Ok(Box::new(OllamaClient::new(config))),
        LLMProvider::OpenAI | LLMProvider::Anthropic => {
            Ok(Box::new(OpenAIClient::new(config)?))
        }
    }
}
