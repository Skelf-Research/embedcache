//! Server configuration module

use anyhow::Result;

/// Server configuration
///
/// This struct holds the server configuration parameters.
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub db_path: String,
    pub db_journal_mode: String,
    pub enabled_models: Vec<String>,
    // LLM configuration for LLM-based chunking
    pub llm_provider: Option<String>,
    pub llm_model: Option<String>,
    pub llm_base_url: Option<String>,
    pub llm_api_key: Option<String>,
    pub llm_timeout: u64,
}

impl ServerConfig {
    /// Load configuration from environment variables
    ///
    /// This function reads configuration from environment variables, using defaults
    /// for any unset variables.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()
                .expect("Invalid SERVER_PORT"),
            db_path: std::env::var("DB_PATH").unwrap_or_else(|_| "cache.db".to_string()),
            db_journal_mode: std::env::var("DB_JOURNAL_MODE").unwrap_or_else(|_| "wal".to_string()),
            enabled_models: std::env::var("ENABLED_MODELS")
                .unwrap_or_else(|_| "AllMiniLML6V2".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            // LLM configuration
            llm_provider: std::env::var("LLM_PROVIDER").ok(),
            llm_model: std::env::var("LLM_MODEL").ok(),
            llm_base_url: std::env::var("LLM_BASE_URL").ok(),
            llm_api_key: std::env::var("LLM_API_KEY").ok(),
            llm_timeout: std::env::var("LLM_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60),
        })
    }

    /// Check if LLM chunking is configured
    pub fn has_llm_config(&self) -> bool {
        self.llm_provider.is_some()
    }
}
