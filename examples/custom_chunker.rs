//! Example of implementing a custom chunking strategy
//!
//! This example demonstrates how to implement and use a custom chunking strategy
//! with the embedcache library.

use embedcache::ContentChunker;
use async_trait::async_trait;

/// A custom chunker that splits text by sentences
pub struct SentenceChunker;

#[async_trait]
impl ContentChunker for SentenceChunker {
    async fn chunk(&self, content: &str, _size: usize) -> Vec<String> {
        // Simple sentence splitting by periods
        // In a real implementation, you might use a more sophisticated NLP library
        content
            .split('.')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| format!("{}.", s)) // Add the period back
            .collect()
    }
    
    fn name(&self) -> &str {
        "sentence"
    }
}

fn main() {
    println!("This example shows how to implement a custom chunking strategy.");
    println!("See the source code for implementation details.");
}