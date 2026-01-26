# Rust API Reference

Complete reference for the EmbedCache Rust library API.

## Crate Structure

```
embedcache
├── cache           # Caching functionality
├── chunking        # Text chunking strategies
│   └── llm         # LLM-based chunkers
├── config          # Configuration types
├── embedding       # Embedding generation
├── handlers        # HTTP request handlers
├── models          # Data types
└── utils           # Utility functions
```

## Traits

### ContentChunker

Trait for implementing custom chunking strategies.

```rust
use async_trait::async_trait;

#[async_trait]
pub trait ContentChunker: Send + Sync {
    /// Chunk content into smaller pieces
    async fn chunk(&self, content: &str, size: usize) -> Vec<String>;

    /// Get the name of this chunker
    fn name(&self) -> &str;
}
```

**Implementations:**

- `WordChunker` - Word-based chunking
- `LLMConceptChunker` - LLM concept chunking
- `LLMIntrospectionChunker` - LLM introspection chunking

### Embedder

Trait for implementing embedding generators.

```rust
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Embedder: Send + Sync {
    /// Generate embeddings for text chunks
    async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>>;
}
```

**Implementations:**

- `FastEmbedder` - FastEmbed-based embedding

---

## Types

### Config

Processing configuration.

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub chunking_type: String,
    pub chunking_size: usize,
    pub embedding_model: String,
}
```

### ProcessedContent

Result of URL processing.

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedContent {
    pub url: String,
    pub config: Config,
    pub chunks: HashMap<usize, String>,
    pub embeddings: HashMap<usize, Vec<f32>>,
    pub error: Option<String>,
}
```

### InputData

Input for URL processing.

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub url: String,
    pub config: Option<Config>,
}
```

### InputDataText

Input for text embedding.

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct InputDataText {
    pub text: Vec<String>,
    pub config: Option<Config>,
}
```

### AppState

Application state container.

```rust
pub struct AppState {
    pub db_pool: Pool,
    pub models: HashMap<String, TextInitOptions>,
    pub chunkers: HashMap<String, Box<dyn ContentChunker + Send + Sync>>,
}
```

### ServerConfig

Server configuration loaded from environment.

```rust
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub db_path: String,
    pub db_journal_mode: String,
    pub enabled_models: Vec<String>,
    pub llm_provider: Option<String>,
    pub llm_model: Option<String>,
    pub llm_base_url: Option<String>,
    pub llm_api_key: Option<String>,
    pub llm_timeout: u64,
}
```

### LLMConfig

LLM provider configuration.

```rust
pub struct LLMConfig {
    pub provider: LLMProvider,
    pub model: String,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub timeout_secs: u64,
}
```

---

## Functions

### Initialization

#### initialize_db_pool

Create and initialize the database connection pool.

```rust
pub async fn initialize_db_pool(config: &ServerConfig) -> Result<Pool>
```

#### initialize_models

Load enabled embedding models.

```rust
pub fn initialize_models(config: &ServerConfig) -> Result<HashMap<String, TextInitOptions>>
```

#### initialize_chunkers

Create available chunker instances.

```rust
pub fn initialize_chunkers(
    llm_config: Option<&LLMConfig>,
) -> HashMap<String, Box<dyn ContentChunker + Send + Sync>>
```

### Utilities

#### get_default_config

Get default processing configuration.

```rust
pub fn get_default_config() -> Config
```

Returns:

```rust
Config {
    chunking_type: "words",
    chunking_size: 512,
    embedding_model: "BGESmallENV15",
}
```

#### generate_hash

Generate cache hash for URL and config.

```rust
pub fn generate_hash(url: &str, config: &Config) -> String
```

#### fetch_content

Fetch and extract content from URL.

```rust
pub async fn fetch_content(url: String) -> Result<String>
```

#### get_embedding_model

Map model name to EmbeddingModel enum.

```rust
pub fn get_embedding_model(model_name: &str) -> Option<EmbeddingModel>
```

### Cache Functions

#### get_from_cache

Retrieve cached content by hash.

```rust
pub async fn get_from_cache(
    pool: &Pool,
    hash: String,
) -> Result<Option<ProcessedContent>, actix_web::Error>
```

#### cache_result

Store processed content in cache.

```rust
pub async fn cache_result(
    pool: &Pool,
    hash: String,
    content: &ProcessedContent,
) -> Result<(), actix_web::Error>
```

---

## Constants

### SUPPORTED_MODELS

List of all supported embedding model names.

```rust
pub const SUPPORTED_MODELS: &[&str] = &[
    "AllMiniLML6V2",
    "AllMiniLML6V2Q",
    // ... 20 more models
];
```

---

## Example Usage

### Basic Embedding

```rust
use embedcache::{FastEmbedder, Embedder};
use fastembed::{InitOptions, EmbeddingModel};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let embedder = FastEmbedder {
        options: InitOptions::new(EmbeddingModel::BGESmallENV15),
    };

    let texts = vec!["Hello, world!".to_string()];
    let embeddings = embedder.embed(&texts).await?;

    println!("Embedding dimensions: {}", embeddings[0].len());
    Ok(())
}
```

### Custom Chunker

```rust
use embedcache::ContentChunker;
use async_trait::async_trait;

struct SentenceChunker;

#[async_trait]
impl ContentChunker for SentenceChunker {
    async fn chunk(&self, content: &str, _size: usize) -> Vec<String> {
        content
            .split('.')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn name(&self) -> &str {
        "sentence"
    }
}
```

### Full Application Setup

```rust
use embedcache::{
    ServerConfig, AppState, initialize_db_pool,
    initialize_models, initialize_chunkers, LLMConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServerConfig::from_env()?;
    let db_pool = initialize_db_pool(&config).await?;
    let models = initialize_models(&config)?;
    let llm_config = LLMConfig::from_server_config(&config);
    let chunkers = initialize_chunkers(llm_config.as_ref());

    let state = AppState { db_pool, models, chunkers };
    println!("Ready with {} models", state.models.len());
    Ok(())
}
```
