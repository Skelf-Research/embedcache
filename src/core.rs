use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use async_sqlite::{JournalMode, PoolBuilder, Pool};
use sha2::{Sha256, Digest};
use anyhow::{Result, Context};
use fastembed::{TextEmbedding, EmbeddingModel, TextInitOptions};
use async_trait::async_trait;
use readability::extractor;
use tokio::task;
use std::collections::HashMap;
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use async_sqlite::rusqlite::Error;

/// Configuration for text processing
///
/// This struct defines the parameters used for chunking and embedding generation.
#[derive(Debug, Serialize, Deserialize, Clone,JsonSchema, ApiComponent)]
pub struct Config {
    pub chunking_type: String,
    pub chunking_size: usize,
    pub embedding_model: String,
}

/// Processed content with embeddings
///
/// This struct contains the results of processing a URL or text, including
/// the chunks and their corresponding embeddings.
#[derive(Debug, Serialize, Deserialize,JsonSchema, ApiComponent)]
pub struct ProcessedContent {
    pub url: String,
    pub config: Config,
    pub chunks: HashMap<usize, String>,
    pub embeddings: HashMap<usize, Vec<f32>>,
    pub error: Option<String>,
}

/// Input data for URL processing
#[derive(Debug, Serialize, Deserialize,JsonSchema, ApiComponent)]
pub struct InputData {
    pub url: String,
    pub config: Option<Config>,
}

/// Input data for text embedding
#[derive(Debug, Serialize, Deserialize,JsonSchema, ApiComponent)]
pub struct InputDataText {
    pub text: Vec<String>,
    pub config: Option<Config>,
}

/// Application state shared across handlers
///
/// Contains the database pool, loaded models, and chunkers.
/// The chunkers are stored in a HashMap where the key is the chunker name
/// and the value is a boxed trait object implementing the ContentChunker trait.
pub struct AppState {
    pub db_pool: Pool,
    pub models: HashMap<String, TextInitOptions>,
    pub chunkers: HashMap<String, Box<dyn ContentChunker + Send + Sync>>,
}

/// Trait for content chunking strategies
///
/// Implementors of this trait can provide custom chunking logic for text processing.
/// The trait is async to allow for complex chunking strategies that might require
/// external services or intensive computation.
#[async_trait]
pub trait ContentChunker: Send + Sync {
    /// Chunk the given content into smaller pieces
    ///
    /// # Arguments
    ///
    /// * `content` - The text content to chunk
    /// * `size` - The desired chunk size (implementation dependent)
    ///
    /// # Returns
    ///
    /// A vector of text chunks
    async fn chunk(&self, content: &str, size: usize) -> Vec<String>;
    
    /// Get the name of this chunker for identification
    ///
    /// # Returns
    ///
    /// A string identifier for this chunker
    fn name(&self) -> &str;
}

/// Trait for embedding generation
///
/// Implementors of this trait can provide custom embedding logic.
#[async_trait]
pub trait Embedder {
    /// Generate embeddings for the given text chunks
    ///
    /// # Arguments
    ///
    /// * `chunks` - The text chunks to embed
    ///
    /// # Returns
    ///
    /// A vector of embeddings, one for each chunk
    async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>>;
}

/// Word-based chunking implementation
///
/// This chunker splits text into chunks based on word boundaries.
pub struct WordChunker;

#[async_trait]
impl ContentChunker for WordChunker {
    async fn chunk(&self, content: &str, size: usize) -> Vec<String> {
        content.split_whitespace()
            .collect::<Vec<&str>>()
            .chunks(size)
            .map(|chunk| chunk.join(" "))
            .collect()
    }
    
    fn name(&self) -> &str {
        "words"
    }
}

/// LLM concept-based chunking (placeholder)
///
/// This is a placeholder for future implementation of LLM-based concept chunking.
pub struct LLMConceptChunker;

#[async_trait]
impl ContentChunker for LLMConceptChunker {
    async fn chunk(&self, content: &str, _size: usize) -> Vec<String> {
        // Placeholder implementation
        // TODO: Implement LLM-based concept chunking
        vec![content.to_string()]
    }
    
    fn name(&self) -> &str {
        "llm-concept"
    }
}

/// LLM introspection-based chunking (placeholder)
///
/// This is a placeholder for future implementation of LLM-based introspection chunking.
pub struct LLMIntrospectionChunker;

#[async_trait]
impl ContentChunker for LLMIntrospectionChunker {
    async fn chunk(&self, content: &str, _size: usize) -> Vec<String> {
        // Placeholder implementation
        // TODO: Implement LLM-based introspection chunking
        vec![content.to_string()]
    }
    
    fn name(&self) -> &str {
        "llm-introspection"
    }
}

/// FastEmbed-based embedding implementation
///
/// This embedder uses the fastembed library to generate embeddings.
pub struct FastEmbedder {
    pub options: TextInitOptions,
}

#[async_trait]
impl Embedder for FastEmbedder {
    async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>> {
        let options = self.options.clone();
        let chunks = chunks.to_vec();

        task::spawn_blocking(move || {
            let mut model = TextEmbedding::try_new(options)?;
            model.embed(chunks, None)
        })
        .await
        .map_err(|e| anyhow::Error::from(e))
        .and_then(|result| result.map_err(|e| anyhow::Error::from(e)))
    }
}

/// Get the default configuration for text processing
///
/// Returns a Config struct with sensible defaults:
/// - chunking_type: "words"
/// - chunking_size: 512
/// - embedding_model: "BGESmallENV15"
pub fn get_default_config() -> Config {
    Config {
        chunking_type: "words".to_string(),
        chunking_size: 512,
        embedding_model: "BGESmallENV15".to_string(),
    }
}

/// Generate a hash for caching purposes
///
/// Creates a SHA-256 hash based on the URL and configuration parameters.
///
/// # Arguments
///
/// * `url` - The URL being processed
/// * `config` - The configuration used for processing
///
/// # Returns
///
/// A hexadecimal string representation of the hash
pub fn generate_hash(url: &str, config: &Config) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url);
    hasher.update(&config.chunking_type);
    hasher.update(config.chunking_size.to_string());
    hasher.update(&config.embedding_model);
    format!("{:x}", hasher.finalize())
}

/// Process text and return embeddings
///
/// This function generates embeddings for a list of text strings.
#[api_operation(summary = "Process a text and return the embeddings")]
pub async fn embed_text(
    input: web::Json<InputDataText>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let config = input.config.clone().unwrap_or_else(get_default_config);
    let options = data.models.get(&config.embedding_model)
        .ok_or_else(|| actix_web::error::ErrorBadRequest(format!("Unsupported embedding model: {}", config.embedding_model)))?;
    let embedder = FastEmbedder { options: options.clone() };
    let embeddings = embedder.embed(&input.text).await.map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(embeddings))
}

/// Process a URL and return processed content
///
/// This function fetches content from a URL, chunks it, and generates embeddings.
#[api_operation(summary = "Process a URL and return processed content")]
pub async fn process_url(
    input: web::Json<InputData>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {

    let config = input.config.clone().unwrap_or_else(get_default_config);
    let hash = generate_hash(&input.url, &config);

    // Check cache
    if let Some(cached_content) = get_from_cache(&data.db_pool, hash.clone()).await? {
        return Ok(HttpResponse::Ok().json(cached_content));
    }

    // Fetch content
    let content = fetch_content(input.url.clone()).await.map_err(actix_web::error::ErrorInternalServerError)?;

    if content == "Failed to scrape content" {
        let processed_content = ProcessedContent {
            url: input.url.clone(),
            config: config.clone(),
            chunks: HashMap::new(),
            embeddings: HashMap::new(),
            error: "Failed to scrape content".to_string().into(),
        };
        return Ok(HttpResponse::Ok().json(processed_content));
    }

    // Process content
    let chunker = data.chunkers.get(&config.chunking_type)
        .ok_or_else(|| actix_web::error::ErrorBadRequest(format!("Unsupported chunking type: {}", config.chunking_type)))?;
    let chunks = chunker.chunk(&content, config.chunking_size).await;

    let options = data.models.get(&config.embedding_model)
        .ok_or_else(|| actix_web::error::ErrorBadRequest(format!("Unsupported embedding model: {}", config.embedding_model)))?;
    let embedder = FastEmbedder { options: options.clone() };
    let embeddings = embedder.embed(&chunks).await.map_err(actix_web::error::ErrorInternalServerError)?;

    let processed_content = ProcessedContent {
        url: input.url.clone(),
        config: config.clone(),
        chunks: chunks.into_iter().enumerate().collect(),
        embeddings: embeddings.into_iter().enumerate().collect(),
        error: None,
    };

    // Cache result
    cache_result(&data.db_pool, hash.clone(), &processed_content).await?;

    Ok(HttpResponse::Ok().json(processed_content))
}

/// Get cached content from the database
///
/// This function retrieves previously processed content from the cache.
pub async fn get_from_cache(pool: &Pool, hash: String) -> Result<Option<ProcessedContent>, actix_web::Error> {
    let result: Option<String> = pool
        .conn(|conn| {
            conn.query_row("SELECT content FROM cache WHERE hash = ?", [hash], |row| row.get(0))
                .map(Some)
                .or_else(|err| match err {
                    Error::QueryReturnedNoRows => Ok(None),
                    _ => Err(err),
                })
        })
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result.map(|json| serde_json::from_str(&json).unwrap()))
}

/// Cache processed content in the database
///
/// This function stores processed content in the cache for future use.
pub async fn cache_result(pool: &Pool, hash: String, content: &ProcessedContent) -> Result<(), actix_web::Error> {
    let json = serde_json::to_string(content).map_err(actix_web::error::ErrorInternalServerError)?;
    
    pool.conn(|conn| {
        conn.execute("INSERT OR REPLACE INTO cache (hash, content) VALUES (?, ?)", [hash, json])
    }).await.map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(())
}

/// Fetch content from a URL
///
/// This function extracts text content from a URL using the readability library.
pub async fn fetch_content(url: String) -> Result<String> {

    task::spawn_blocking(move || {

        extractor::scrape(&url)
            .map(|product| product.content)
            .unwrap_or_else(|_| String::from("Failed to scrape content"))
        
    }).await.context("Failed to fetch content")

}

/// List supported features
///
/// This function returns a list of supported chunking types and embedding models.
#[api_operation(summary = "Get a list of supported features")]
pub async fn list_supported_features() -> HttpResponse {
    let supported_features = json!({
        "chunking_types": ["words", "llm-concept", "llm-introspection"],
        "embedding_models": [
            "AllMiniLML6V2",
            "AllMiniLML6V2Q",
            "AllMiniLML12V2",
            "AllMiniLML12V2Q",
            "BGEBaseENV15",
            "BGEBaseENV15Q",
            "BGELargeENV15",
            "BGELargeENV15Q",
            "BGESmallENV15",
            "BGESmallENV15Q",
            "NomicEmbedTextV1",
            "NomicEmbedTextV15",
            "NomicEmbedTextV15Q",
            "ParaphraseMLMiniLML12V2",
            "ParaphraseMLMiniLML12V2Q",
            "ParaphraseMLMpnetBaseV2",
            "BGESmallZHV15",
            "MultilingualE5Small",
            "MultilingualE5Base",
            "MultilingualE5Large",
            "MxbaiEmbedLargeV1",
            "MxbaiEmbedLargeV1Q"
        ]
    });

    HttpResponse::Ok().json(supported_features)
}

/// Get embedding model by name
///
/// This function maps a model name string to an EmbeddingModel enum.
pub fn get_embedding_model(model_name: &str) -> Option<EmbeddingModel> {
    match model_name {
        "AllMiniLML6V2" => Some(EmbeddingModel::AllMiniLML6V2),
        "AllMiniLML6V2Q" => Some(EmbeddingModel::AllMiniLML6V2Q),
        "AllMiniLML12V2" => Some(EmbeddingModel::AllMiniLML12V2),
        "AllMiniLML12V2Q" => Some(EmbeddingModel::AllMiniLML12V2Q),
        "BGEBaseENV15" => Some(EmbeddingModel::BGEBaseENV15),
        "BGEBaseENV15Q" => Some(EmbeddingModel::BGEBaseENV15Q),
        "BGELargeENV15" => Some(EmbeddingModel::BGELargeENV15),
        "BGELargeENV15Q" => Some(EmbeddingModel::BGELargeENV15Q),
        "BGESmallENV15" => Some(EmbeddingModel::BGESmallENV15),
        "BGESmallENV15Q" => Some(EmbeddingModel::BGESmallENV15Q),
        "NomicEmbedTextV1" => Some(EmbeddingModel::NomicEmbedTextV1), 
        "NomicEmbedTextV15" => Some(EmbeddingModel::NomicEmbedTextV15),
        "NomicEmbedTextV15Q" => Some(EmbeddingModel::NomicEmbedTextV15Q),
        "ParaphraseMLMiniLML12V2" => Some(EmbeddingModel::ParaphraseMLMiniLML12V2),
        "ParaphraseMLMiniLML12V2Q" => Some(EmbeddingModel::ParaphraseMLMiniLML12V2Q),
        "ParaphraseMLMpnetBaseV2" => Some(EmbeddingModel::ParaphraseMLMpnetBaseV2),
        "BGESmallZHV15" => Some(EmbeddingModel::BGESmallZHV15),
        "MultilingualE5Small" => Some(EmbeddingModel::MultilingualE5Small),
        "MultilingualE5Base" => Some(EmbeddingModel::MultilingualE5Base),
        "MultilingualE5Large" => Some(EmbeddingModel::MultilingualE5Large),
        "MxbaiEmbedLargeV1" => Some(EmbeddingModel::MxbaiEmbedLargeV1),
        "MxbaiEmbedLargeV1Q" => Some(EmbeddingModel::MxbaiEmbedLargeV1Q),
        _ => None,
    }
}

/// Server configuration
///
/// This struct holds the server configuration parameters.
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub db_path: String,
    pub db_journal_mode: String,
    pub enabled_models: Vec<String>,
}

impl ServerConfig {
    /// Load configuration from environment variables
    ///
    /// This function reads configuration from environment variables, using defaults
    /// for any unset variables.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("SERVER_PORT").unwrap_or_else(|_| "8081".to_string())
                .parse()
                .expect("Invalid SERVER_PORT"),
            db_path: std::env::var("DB_PATH").unwrap_or_else(|_| "cache.db".to_string()),
            db_journal_mode: std::env::var("DB_JOURNAL_MODE").unwrap_or_else(|_| "wal".to_string()),
            enabled_models: std::env::var("ENABLED_MODELS")
                .unwrap_or_else(|_| "AllMiniLML6V2".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        })
    }
}

/// Initialize the database pool
///
/// This function creates and initializes the database connection pool.
pub async fn initialize_db_pool(config: &ServerConfig) -> Result<Pool, anyhow::Error> {
    let db_pool = PoolBuilder::new()
        .path(&config.db_path)
        .journal_mode(match config.db_journal_mode.to_lowercase().as_str() {
            "wal" => JournalMode::Wal,
            "truncate" => JournalMode::Truncate,
            "persist" => JournalMode::Persist,
            _ => JournalMode::Wal,
        })
        .open()
        .await
        .expect("Failed to create database pool");
    
    // Initialize database
    db_pool.conn(|conn| {
        conn.execute("CREATE TABLE IF NOT EXISTS cache (hash TEXT PRIMARY KEY, content TEXT)", [])
    }).await.expect("Failed to create cache table");
    
    Ok(db_pool)
}

/// Initialize embedding models
///
/// Initialize embedding models
///
/// This function creates the available embedding models.
pub fn initialize_models(config: &ServerConfig) -> Result<HashMap<String, TextInitOptions>, anyhow::Error> {
    let mut models = HashMap::new();
    
    for name in &config.enabled_models {
        let model_name = get_embedding_model(name).expect("Invalid model name");
        let options = TextInitOptions::new(model_name)
            .with_show_download_progress(true);
        models.insert(name.to_string(), options);
    }
    
    Ok(models)
}

/// Initialize chunkers
///
/// This function creates the available chunking strategies.
pub fn initialize_chunkers() -> HashMap<String, Box<dyn ContentChunker + Send + Sync>> {
    let mut chunkers: HashMap<String, Box<dyn ContentChunker + Send + Sync>> = HashMap::new();
    let word_chunker = WordChunker;
    chunkers.insert(word_chunker.name().to_string(), Box::new(word_chunker));
    
    let llm_concept_chunker = LLMConceptChunker;
    chunkers.insert(llm_concept_chunker.name().to_string(), Box::new(llm_concept_chunker));
    
    let llm_introspection_chunker = LLMIntrospectionChunker;
    chunkers.insert(llm_introspection_chunker.name().to_string(), Box::new(llm_introspection_chunker));
    
    chunkers
}