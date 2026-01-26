# Custom Chunkers

This guide explains how to implement custom text chunking strategies.

## The ContentChunker Trait

All chunkers implement the `ContentChunker` trait:

```rust
use async_trait::async_trait;

#[async_trait]
pub trait ContentChunker: Send + Sync {
    async fn chunk(&self, content: &str, size: usize) -> Vec<String>;
    fn name(&self) -> &str;
}
```

## Basic Example: Sentence Chunker

```rust
use embedcache::ContentChunker;
use async_trait::async_trait;

pub struct SentenceChunker;

#[async_trait]
impl ContentChunker for SentenceChunker {
    async fn chunk(&self, content: &str, _size: usize) -> Vec<String> {
        content
            .split('.')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| format!("{}.", s))
            .collect()
    }

    fn name(&self) -> &str {
        "sentence"
    }
}
```

## Advanced Example: Paragraph Chunker

```rust
use embedcache::ContentChunker;
use async_trait::async_trait;

pub struct ParagraphChunker {
    min_length: usize,
}

impl ParagraphChunker {
    pub fn new(min_length: usize) -> Self {
        Self { min_length }
    }
}

#[async_trait]
impl ContentChunker for ParagraphChunker {
    async fn chunk(&self, content: &str, _size: usize) -> Vec<String> {
        content
            .split("\n\n")
            .map(|p| p.trim().to_string())
            .filter(|p| p.len() >= self.min_length)
            .collect()
    }

    fn name(&self) -> &str {
        "paragraph"
    }
}
```

## Async Chunking

The trait is async to support chunkers that need external services:

```rust
use embedcache::ContentChunker;
use async_trait::async_trait;

pub struct ApiBasedChunker {
    api_url: String,
    client: reqwest::Client,
}

#[async_trait]
impl ContentChunker for ApiBasedChunker {
    async fn chunk(&self, content: &str, size: usize) -> Vec<String> {
        // Call external API
        match self.client
            .post(&self.api_url)
            .json(&serde_json::json!({
                "content": content,
                "chunk_size": size
            }))
            .send()
            .await
        {
            Ok(response) => {
                response.json::<Vec<String>>().await.unwrap_or_else(|_| {
                    // Fallback to simple chunking
                    vec![content.to_string()]
                })
            }
            Err(_) => vec![content.to_string()],
        }
    }

    fn name(&self) -> &str {
        "api-chunker"
    }
}
```

## Registering Custom Chunkers

Add your chunker to the chunker map:

```rust
use std::collections::HashMap;
use embedcache::{ContentChunker, WordChunker, initialize_chunkers, LLMConfig};

fn create_chunkers_with_custom(
    llm_config: Option<&LLMConfig>,
) -> HashMap<String, Box<dyn ContentChunker + Send + Sync>> {
    // Start with default chunkers
    let mut chunkers = initialize_chunkers(llm_config);

    // Add custom chunker
    let sentence_chunker = SentenceChunker;
    chunkers.insert(
        sentence_chunker.name().to_string(),
        Box::new(sentence_chunker),
    );

    chunkers
}
```

## Using Custom Chunkers in API

Once registered, use the chunker name in API requests:

```bash
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{
    "text": ["Your text here..."],
    "config": {
      "chunking_type": "sentence",
      "chunking_size": 256,
      "embedding_model": "BGESmallENV15"
    }
  }'
```

## Best Practices

### 1. Handle Edge Cases

```rust
async fn chunk(&self, content: &str, size: usize) -> Vec<String> {
    // Handle empty content
    if content.trim().is_empty() {
        return vec![];
    }

    // Handle very short content
    if content.len() < size {
        return vec![content.to_string()];
    }

    // Normal chunking logic...
}
```

### 2. Preserve Context

```rust
async fn chunk(&self, content: &str, size: usize) -> Vec<String> {
    let chunks: Vec<String> = /* your chunking logic */;

    // Add overlap for context
    let overlap = size / 4;
    chunks.windows(2)
        .map(|window| format!("{} {}", window[0], window[1]))
        .collect()
}
```

### 3. Provide Fallbacks

```rust
async fn chunk(&self, content: &str, size: usize) -> Vec<String> {
    match self.complex_chunking(content, size).await {
        Ok(chunks) if !chunks.is_empty() => chunks,
        _ => {
            // Fallback to simple word chunking
            content
                .split_whitespace()
                .collect::<Vec<&str>>()
                .chunks(size)
                .map(|c| c.join(" "))
                .collect()
        }
    }
}
```

## Testing Chunkers

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sentence_chunker() {
        let chunker = SentenceChunker;
        let content = "First sentence. Second sentence. Third sentence.";

        let chunks = chunker.chunk(content, 10).await;

        assert_eq!(chunks.len(), 3);
        assert!(chunks[0].ends_with('.'));
    }

    #[tokio::test]
    async fn test_empty_content() {
        let chunker = SentenceChunker;
        let chunks = chunker.chunk("", 10).await;

        assert!(chunks.is_empty());
    }
}
```
