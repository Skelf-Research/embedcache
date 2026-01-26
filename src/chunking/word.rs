//! Word-based chunking implementation

use async_trait::async_trait;
use super::ContentChunker;

/// Word-based chunking implementation
///
/// This chunker splits text into chunks based on word boundaries.
pub struct WordChunker;

#[async_trait]
impl ContentChunker for WordChunker {
    async fn chunk(&self, content: &str, size: usize) -> Vec<String> {
        content
            .split_whitespace()
            .collect::<Vec<&str>>()
            .chunks(size)
            .map(|chunk| chunk.join(" "))
            .collect()
    }

    fn name(&self) -> &str {
        "words"
    }
}
