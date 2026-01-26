//! LLM concept-based chunking implementation

use async_trait::async_trait;
use std::sync::Arc;

use super::client::LLMClient;
use crate::chunking::ContentChunker;

/// LLM concept-based chunking
///
/// This chunker uses an LLM to identify semantic concept boundaries in text,
/// creating chunks that are semantically coherent.
pub struct LLMConceptChunker {
    client: Arc<dyn LLMClient>,
}

impl LLMConceptChunker {
    pub fn new(client: Arc<dyn LLMClient>) -> Self {
        Self { client }
    }

    fn parse_chunks(&self, response: &str) -> Option<Vec<String>> {
        // Try to extract JSON array from the response
        let trimmed = response.trim();

        // Handle markdown code blocks
        let json_str = if trimmed.starts_with("```") {
            trimmed
                .lines()
                .skip(1)
                .take_while(|line| !line.starts_with("```"))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            trimmed.to_string()
        };

        // Try to find JSON array in the response
        let start = json_str.find('[')?;
        let end = json_str.rfind(']')? + 1;
        let array_str = &json_str[start..end];

        serde_json::from_str::<Vec<String>>(array_str).ok()
    }

    fn fallback_chunk(&self, content: &str, size: usize) -> Vec<String> {
        content
            .split_whitespace()
            .collect::<Vec<&str>>()
            .chunks(size)
            .map(|chunk| chunk.join(" "))
            .collect()
    }
}

#[async_trait]
impl ContentChunker for LLMConceptChunker {
    async fn chunk(&self, content: &str, size: usize) -> Vec<String> {
        // For very short content, just return as-is
        if content.split_whitespace().count() <= size {
            return vec![content.to_string()];
        }

        let prompt = format!(
            r#"You are a text segmentation assistant. Your task is to divide the following text into logical chunks based on semantic concepts and topic boundaries.

Rules:
1. Each chunk should contain a complete concept or topic
2. Target approximately {size} words per chunk, but prioritize semantic coherence
3. Return ONLY a JSON array of strings, each string being one chunk
4. Preserve the original text exactly - do not summarize or modify
5. Do not include any explanation, just the JSON array

Text to segment:
---
{content}
---

Return the chunks as a JSON array:"#,
            size = size,
            content = content
        );

        match self.client.complete(&prompt).await {
            Ok(response) => {
                if let Some(chunks) = self.parse_chunks(&response) {
                    if !chunks.is_empty() {
                        return chunks;
                    }
                }
                eprintln!("LLM concept chunking: Failed to parse response, falling back to word chunking");
                self.fallback_chunk(content, size)
            }
            Err(e) => {
                eprintln!("LLM concept chunking failed: {e}, falling back to word chunking");
                self.fallback_chunk(content, size)
            }
        }
    }

    fn name(&self) -> &str {
        "llm-concept"
    }
}
