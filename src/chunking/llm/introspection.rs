//! LLM introspection-based chunking implementation

use async_trait::async_trait;
use std::sync::Arc;

use super::client::LLMClient;
use crate::chunking::ContentChunker;

/// LLM introspection-based chunking
///
/// This chunker uses a two-step process:
/// 1. Analyze the document structure and identify key topics
/// 2. Use that analysis to create optimized chunks for embedding generation
pub struct LLMIntrospectionChunker {
    client: Arc<dyn LLMClient>,
}

impl LLMIntrospectionChunker {
    pub fn new(client: Arc<dyn LLMClient>) -> Self {
        Self { client }
    }

    async fn analyze_document(&self, content: &str, size: usize) -> Option<String> {
        let prompt = format!(
            r#"Analyze the following text and identify:
1. Main topics/sections present in the text
2. Key concepts mentioned
3. Logical breaking points where the topic changes
4. Recommended chunk boundaries for embedding generation

Target chunk size: approximately {size} words each.

Text:
---
{content}
---

Provide a brief analysis (2-3 sentences) of the document structure and recommended chunking strategy:"#,
            size = size,
            content = content
        );

        self.client.complete(&prompt).await.ok()
    }

    fn parse_chunks(&self, response: &str) -> Option<Vec<String>> {
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
impl ContentChunker for LLMIntrospectionChunker {
    async fn chunk(&self, content: &str, size: usize) -> Vec<String> {
        // For very short content, just return as-is
        if content.split_whitespace().count() <= size {
            return vec![content.to_string()];
        }

        // Step 1: Analyze the document
        let analysis = match self.analyze_document(content, size).await {
            Some(a) => a,
            None => {
                eprintln!("LLM introspection: Analysis failed, falling back to word chunking");
                return self.fallback_chunk(content, size);
            }
        };

        // Step 2: Use analysis to create optimized chunks
        let prompt = format!(
            r#"Based on this document analysis:
{analysis}

Now segment the following text into chunks. Each chunk should:
1. Be self-contained enough to generate meaningful embeddings
2. Include enough context for semantic understanding
3. Not split mid-sentence or mid-concept
4. Target approximately {size} words per chunk

Text:
---
{content}
---

Return ONLY a JSON array of chunk strings, no explanation:"#,
            analysis = analysis,
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
                eprintln!("LLM introspection chunking: Failed to parse response, falling back to word chunking");
                self.fallback_chunk(content, size)
            }
            Err(e) => {
                eprintln!("LLM introspection chunking failed: {e}, falling back to word chunking");
                self.fallback_chunk(content, size)
            }
        }
    }

    fn name(&self) -> &str {
        "llm-introspection"
    }
}
