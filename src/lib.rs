//! EmbedCache - High-performance text embedding library with caching capabilities
//!
//! This library provides functionality for generating text embeddings with various
//! state-of-the-art models and caching the results for improved performance.
//!
//! ## Features
//!
//! - Multiple embedding models (BGE, MiniLM, Nomic, etc.)
//! - Modular text chunking strategies with extensible trait-based architecture
//! - LLM-based intelligent chunking (concept and introspection modes)
//! - SQLite-based caching
//! - Asynchronous operation
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! embedcache = "0.1.0"
//! ```
//!
//! ## Examples
//!
//! ### Using as a library
//!
//! ```no_run
//! use embedcache::{FastEmbedder, Embedder};
//! use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let model = TextEmbedding::try_new(InitOptions {
//!     model_name: EmbeddingModel::BGESmallENV15,
//!     show_download_progress: true,
//!     ..Default::default()
//! })?;
//!
//! let embedder = FastEmbedder {
//!     options: InitOptions::new(EmbeddingModel::BGESmallENV15),
//! };
//!
//! let texts = vec![
//!     "This is an example sentence.".to_string(),
//!     "Another example sentence for embedding.".to_string(),
//! ];
//!
//! let embeddings = embedder.embed(&texts).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Implementing custom chunking strategies
//!
//! ```no_run
//! use embedcache::ContentChunker;
//! use async_trait::async_trait;
//!
//! struct MyCustomChunker;
//!
//! #[async_trait]
//! impl ContentChunker for MyCustomChunker {
//!     async fn chunk(&self, content: &str, size: usize) -> Vec<String> {
//!         // Your custom chunking logic here
//!         vec![content.to_string()]
//!     }
//!
//!     fn name(&self) -> &str {
//!         "my-custom-chunker"
//!     }
//! }
//! ```
//!
//! ### Running the embedcache service
//!
//! The library also includes a binary that can be run as a standalone service:
//!
//! ```bash
//! cargo install embedcache
//! embedcache
//! ```

pub mod cache;
pub mod chunking;
pub mod config;
pub mod embedding;
pub mod handlers;
pub mod models;
pub mod utils;

// Re-export commonly used types at the crate root for convenience
pub use cache::{cache_result, get_from_cache, initialize_db_pool};
pub use chunking::{ContentChunker, LLMConceptChunker, LLMConfig, LLMIntrospectionChunker, WordChunker};
pub use config::ServerConfig;
pub use embedding::{get_embedding_model, initialize_models, Embedder, FastEmbedder, SUPPORTED_MODELS};
pub use handlers::{embed_text, list_supported_features, process_url};
pub use models::{get_default_config, AppState, Config, InputData, InputDataText, ProcessedContent};
pub use utils::{fetch_content, generate_hash};

use std::collections::HashMap;
use std::sync::Arc;

/// Initialize chunkers
///
/// This function creates the available chunking strategies.
/// If LLM configuration is provided, LLM-based chunkers will be enabled.
pub fn initialize_chunkers(
    llm_config: Option<&LLMConfig>,
) -> HashMap<String, Box<dyn ContentChunker + Send + Sync>> {
    let mut chunkers: HashMap<String, Box<dyn ContentChunker + Send + Sync>> = HashMap::new();

    // Word chunker is always available
    let word_chunker = WordChunker;
    chunkers.insert(word_chunker.name().to_string(), Box::new(word_chunker));

    // LLM chunkers are only available if LLM is configured
    if let Some(config) = llm_config {
        match chunking::llm::create_llm_client(config) {
            Ok(client) => {
                let client = Arc::from(client);

                let concept_chunker = LLMConceptChunker::new(Arc::clone(&client));
                chunkers.insert(concept_chunker.name().to_string(), Box::new(concept_chunker));

                let introspection_chunker = LLMIntrospectionChunker::new(Arc::clone(&client));
                chunkers.insert(
                    introspection_chunker.name().to_string(),
                    Box::new(introspection_chunker),
                );

                println!("LLM chunkers initialized successfully");
            }
            Err(e) => {
                eprintln!("Failed to initialize LLM client: {e}. LLM chunkers disabled.");
            }
        }
    } else {
        println!("LLM not configured. Only word chunking available.");
    }

    chunkers
}
