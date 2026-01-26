//! FastEmbed-based embedding implementation

use anyhow::Result;
use async_trait::async_trait;
use fastembed::{TextEmbedding, TextInitOptions};
use tokio::task;

use super::Embedder;

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
        .map_err(anyhow::Error::from)
        .and_then(|result| result.map_err(anyhow::Error::from))
    }
}
