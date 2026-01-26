# Library Usage

This guide covers how to use EmbedCache as a Rust library in your applications.

## Adding the Dependency

Add EmbedCache to your `Cargo.toml`:

```toml
[dependencies]
embedcache = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Basic Embedding Generation

```rust
use embedcache::{FastEmbedder, Embedder};
use fastembed::{InitOptions, EmbeddingModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an embedder with a specific model
    let embedder = FastEmbedder {
        options: InitOptions::new(EmbeddingModel::BGESmallENV15),
    };

    // Texts to embed
    let texts = vec![
        "Machine learning transforms data into insights.".to_string(),
        "Natural language processing enables text understanding.".to_string(),
    ];

    // Generate embeddings
    let embeddings = embedder.embed(&texts).await?;

    // Process results
    for (i, embedding) in embeddings.iter().enumerate() {
        println!("Text {}: {} dimensions", i, embedding.len());
        println!("First 5 values: {:?}", &embedding[..5]);
    }

    Ok(())
}
```

## Using Chunkers

```rust
use embedcache::{ContentChunker, WordChunker};

#[tokio::main]
async fn main() {
    let chunker = WordChunker;

    let text = "This is a long document that needs to be chunked into smaller pieces for processing.";

    // Chunk with 5 words per chunk
    let chunks = chunker.chunk(text, 5).await;

    for (i, chunk) in chunks.iter().enumerate() {
        println!("Chunk {}: {}", i, chunk);
    }
}
```

## Full Application State Setup

For more complex usage, you can set up the full application state:

```rust
use embedcache::{
    ServerConfig, AppState, initialize_db_pool, initialize_models,
    initialize_chunkers, LLMConfig,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = ServerConfig::from_env()?;

    // Initialize components
    let db_pool = initialize_db_pool(&config).await?;
    let models = initialize_models(&config)?;
    let llm_config = LLMConfig::from_server_config(&config);
    let chunkers = initialize_chunkers(llm_config.as_ref());

    // Create application state
    let app_state = Arc::new(AppState {
        db_pool,
        models,
        chunkers,
    });

    // Use the state...
    println!("Initialized with {} models and {} chunkers",
             app_state.models.len(),
             app_state.chunkers.len());

    Ok(())
}
```

## Working with Configuration

```rust
use embedcache::{Config, get_default_config};

fn main() {
    // Get default configuration
    let default_config = get_default_config();
    println!("Default model: {}", default_config.embedding_model);

    // Create custom configuration
    let custom_config = Config {
        chunking_type: "llm-concept".to_string(),
        chunking_size: 256,
        embedding_model: "BGEBaseENV15".to_string(),
    };

    println!("Custom config: {:?}", custom_config);
}
```

## Using the Cache

```rust
use embedcache::{
    ServerConfig, initialize_db_pool, get_from_cache, cache_result,
    ProcessedContent, Config,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::from_env()?;
    let pool = initialize_db_pool(&config).await?;

    // Check cache
    let hash = "example_hash".to_string();
    if let Some(cached) = get_from_cache(&pool, hash.clone()).await? {
        println!("Found cached content: {:?}", cached.url);
    } else {
        // Create and cache new content
        let content = ProcessedContent {
            url: "https://example.com".to_string(),
            config: Config {
                chunking_type: "words".to_string(),
                chunking_size: 512,
                embedding_model: "BGESmallENV15".to_string(),
            },
            chunks: HashMap::from([(0, "Example chunk".to_string())]),
            embeddings: HashMap::from([(0, vec![0.1, 0.2, 0.3])]),
            error: None,
        };

        cache_result(&pool, hash, &content).await?;
        println!("Cached new content");
    }

    Ok(())
}
```

## Error Handling

EmbedCache uses `anyhow` for error handling:

```rust
use embedcache::{FastEmbedder, Embedder};
use fastembed::{InitOptions, EmbeddingModel};

#[tokio::main]
async fn main() {
    let embedder = FastEmbedder {
        options: InitOptions::new(EmbeddingModel::BGESmallENV15),
    };

    let texts = vec!["Test text".to_string()];

    match embedder.embed(&texts).await {
        Ok(embeddings) => {
            println!("Success: {} embeddings generated", embeddings.len());
        }
        Err(e) => {
            eprintln!("Error generating embeddings: {}", e);
        }
    }
}
```

## Available Types

### Core Types

| Type | Description |
|------|-------------|
| `Config` | Processing configuration |
| `ProcessedContent` | Results of URL processing |
| `InputData` | Input for URL processing |
| `InputDataText` | Input for text embedding |
| `AppState` | Application state container |
| `ServerConfig` | Server configuration |

### Traits

| Trait | Description |
|-------|-------------|
| `ContentChunker` | Interface for chunking strategies |
| `Embedder` | Interface for embedding generators |

### Implementations

| Type | Description |
|------|-------------|
| `WordChunker` | Word-based chunking |
| `LLMConceptChunker` | LLM concept chunking |
| `LLMIntrospectionChunker` | LLM introspection chunking |
| `FastEmbedder` | FastEmbed-based embedder |
