//! Embedding generation module
//!
//! This module provides traits and implementations for generating text embeddings.

mod fastembed;
mod registry;

use anyhow::Result;
use async_trait::async_trait;

pub use self::fastembed::FastEmbedder;
pub use registry::{get_embedding_model, initialize_models, SUPPORTED_MODELS};

/// Trait for embedding generation
///
/// Implementors of this trait can provide custom embedding logic.
#[async_trait]
pub trait Embedder: Send + Sync {
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
