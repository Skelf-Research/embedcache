//! Content chunking module
//!
//! This module provides traits and implementations for text chunking strategies.

mod word;
pub mod llm;

use async_trait::async_trait;

pub use word::WordChunker;
pub use llm::{LLMConceptChunker, LLMIntrospectionChunker, LLMConfig};

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
