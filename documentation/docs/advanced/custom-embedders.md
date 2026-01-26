# Custom Embedders

This guide explains how to implement custom embedding providers.

## The Embedder Trait

All embedders implement the `Embedder` trait:

```rust
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Embedder: Send + Sync {
    async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>>;
}
```

## Basic Example: OpenAI Embedder

```rust
use embedcache::Embedder;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct OpenAIEmbedder {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl OpenAIEmbedder {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

#[async_trait]
impl Embedder for OpenAIEmbedder {
    async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            input: chunks.to_vec(),
        };

        let response = self.client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .json::<OpenAIResponse>()
            .await?;

        Ok(response.data.into_iter().map(|d| d.embedding).collect())
    }
}
```

## Batch Processing Embedder

For APIs with rate limits or batch size restrictions:

```rust
use embedcache::Embedder;
use anyhow::Result;
use async_trait::async_trait;

pub struct BatchEmbedder {
    inner: Box<dyn Embedder>,
    batch_size: usize,
}

impl BatchEmbedder {
    pub fn new(inner: Box<dyn Embedder>, batch_size: usize) -> Self {
        Self { inner, batch_size }
    }
}

#[async_trait]
impl Embedder for BatchEmbedder {
    async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut all_embeddings = Vec::new();

        for batch in chunks.chunks(self.batch_size) {
            let batch_embeddings = self.inner.embed(batch).await?;
            all_embeddings.extend(batch_embeddings);

            // Optional: Add delay between batches
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        Ok(all_embeddings)
    }
}
```

## Caching Embedder

Wrap any embedder with caching:

```rust
use embedcache::Embedder;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;
use sha2::{Sha256, Digest};

pub struct CachingEmbedder {
    inner: Box<dyn Embedder>,
    cache: RwLock<HashMap<String, Vec<f32>>>,
}

impl CachingEmbedder {
    pub fn new(inner: Box<dyn Embedder>) -> Self {
        Self {
            inner,
            cache: RwLock::new(HashMap::new()),
        }
    }

    fn hash_text(text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[async_trait]
impl Embedder for CachingEmbedder {
    async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut results = Vec::with_capacity(chunks.len());
        let mut to_embed = Vec::new();
        let mut indices = Vec::new();

        // Check cache
        {
            let cache = self.cache.read().unwrap();
            for (i, chunk) in chunks.iter().enumerate() {
                let hash = Self::hash_text(chunk);
                if let Some(embedding) = cache.get(&hash) {
                    results.push(Some(embedding.clone()));
                } else {
                    results.push(None);
                    to_embed.push(chunk.clone());
                    indices.push(i);
                }
            }
        }

        // Embed cache misses
        if !to_embed.is_empty() {
            let new_embeddings = self.inner.embed(&to_embed).await?;

            let mut cache = self.cache.write().unwrap();
            for (chunk, embedding) in to_embed.iter().zip(new_embeddings.iter()) {
                let hash = Self::hash_text(chunk);
                cache.insert(hash, embedding.clone());
            }

            for (idx, embedding) in indices.into_iter().zip(new_embeddings) {
                results[idx] = Some(embedding);
            }
        }

        Ok(results.into_iter().map(|r| r.unwrap()).collect())
    }
}
```

## Fallback Embedder

Use multiple embedders with fallback:

```rust
use embedcache::Embedder;
use anyhow::Result;
use async_trait::async_trait;

pub struct FallbackEmbedder {
    primary: Box<dyn Embedder>,
    fallback: Box<dyn Embedder>,
}

impl FallbackEmbedder {
    pub fn new(primary: Box<dyn Embedder>, fallback: Box<dyn Embedder>) -> Self {
        Self { primary, fallback }
    }
}

#[async_trait]
impl Embedder for FallbackEmbedder {
    async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>> {
        match self.primary.embed(chunks).await {
            Ok(embeddings) => Ok(embeddings),
            Err(e) => {
                eprintln!("Primary embedder failed: {}, using fallback", e);
                self.fallback.embed(chunks).await
            }
        }
    }
}
```

## Using Custom Embedders

Replace the default embedder in handlers:

```rust
use embedcache::{Embedder, InputDataText, get_default_config};

async fn embed_with_custom(
    input: InputDataText,
    embedder: &dyn Embedder,
) -> Result<Vec<Vec<f32>>, anyhow::Error> {
    let config = input.config.unwrap_or_else(get_default_config);

    // Use custom embedder
    let embeddings = embedder.embed(&input.text).await?;

    Ok(embeddings)
}
```

## Best Practices

### 1. Handle Empty Input

```rust
async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>> {
    if chunks.is_empty() {
        return Ok(vec![]);
    }
    // ... rest of implementation
}
```

### 2. Validate Dimensions

```rust
async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>> {
    let embeddings = self.inner_embed(chunks).await?;

    // Verify all embeddings have same dimension
    if let Some(first) = embeddings.first() {
        let expected_dim = first.len();
        for (i, emb) in embeddings.iter().enumerate() {
            if emb.len() != expected_dim {
                return Err(anyhow::anyhow!(
                    "Embedding {} has {} dimensions, expected {}",
                    i, emb.len(), expected_dim
                ));
            }
        }
    }

    Ok(embeddings)
}
```

### 3. Implement Timeouts

```rust
async fn embed(&self, chunks: &[String]) -> Result<Vec<Vec<f32>>> {
    tokio::time::timeout(
        std::time::Duration::from_secs(30),
        self.inner_embed(chunks)
    )
    .await
    .map_err(|_| anyhow::anyhow!("Embedding request timed out"))?
}
```

## Testing Embedders

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_custom_embedder() {
        let embedder = MyCustomEmbedder::new();
        let texts = vec!["Hello, world!".to_string()];

        let embeddings = embedder.embed(&texts).await.unwrap();

        assert_eq!(embeddings.len(), 1);
        assert!(!embeddings[0].is_empty());
    }

    #[tokio::test]
    async fn test_empty_input() {
        let embedder = MyCustomEmbedder::new();
        let texts: Vec<String> = vec![];

        let embeddings = embedder.embed(&texts).await.unwrap();

        assert!(embeddings.is_empty());
    }
}
```
