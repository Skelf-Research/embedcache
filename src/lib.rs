//! EmbedCache - High-performance text embedding library with caching capabilities
//!
//! This library provides functionality for generating text embeddings with various
//! state-of-the-art models and caching the results for improved performance.
//!
//! ## Features
//!
//! - Multiple embedding models (BGE, MiniLM, Nomic, etc.)
//! - Modular text chunking strategies with extensible trait-based architecture
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
//! use embedcache::{ContentChunker};
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

pub mod core;

// Re-export the core module contents at the crate root for convenience
pub use crate::core::*;