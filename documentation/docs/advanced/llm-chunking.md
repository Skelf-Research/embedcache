# LLM Chunking

This guide covers how to configure and use LLM-based chunking strategies.

## Overview

EmbedCache provides two LLM-powered chunking strategies:

1. **LLM Concept Chunking** - Uses LLM to identify semantic concept boundaries
2. **LLM Introspection Chunking** - Two-step analysis and chunking process

Both strategies fall back to word chunking if LLM calls fail.

## Supported Providers

| Provider | Configuration |
|----------|---------------|
| Ollama | Local LLM server |
| OpenAI | OpenAI API |
| Anthropic | Claude API |

## Configuration

### Ollama (Recommended for Development)

1. Install Ollama:

```bash
# macOS/Linux
curl -fsSL https://ollama.com/install.sh | sh

# Pull a model
ollama pull llama3
```

2. Configure EmbedCache:

```bash
# .env file
LLM_PROVIDER=ollama
LLM_MODEL=llama3
LLM_BASE_URL=http://localhost:11434
LLM_TIMEOUT=60
```

### OpenAI

```bash
# .env file
LLM_PROVIDER=openai
LLM_MODEL=gpt-4o-mini
LLM_API_KEY=sk-your-api-key-here
LLM_TIMEOUT=60
```

### Anthropic

```bash
# .env file
LLM_PROVIDER=anthropic
LLM_MODEL=claude-3-haiku-20240307
LLM_API_KEY=sk-ant-your-api-key-here
LLM_BASE_URL=https://api.anthropic.com/v1
LLM_TIMEOUT=60
```

## Using LLM Chunking

### Via REST API

```bash
# LLM Concept Chunking
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{
    "text": ["Your long document text here..."],
    "config": {
      "chunking_type": "llm-concept",
      "chunking_size": 256,
      "embedding_model": "BGESmallENV15"
    }
  }'

# LLM Introspection Chunking
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{
    "text": ["Your long document text here..."],
    "config": {
      "chunking_type": "llm-introspection",
      "chunking_size": 256,
      "embedding_model": "BGESmallENV15"
    }
  }'
```

### Via Rust Library

```rust
use embedcache::{
    LLMConceptChunker, LLMIntrospectionChunker, LLMConfig, LLMProvider,
    ContentChunker,
};
use embedcache::chunking::llm::create_llm_client;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure LLM
    let config = LLMConfig {
        provider: LLMProvider::Ollama,
        model: "llama3".to_string(),
        base_url: Some("http://localhost:11434".to_string()),
        api_key: None,
        timeout_secs: 60,
    };

    // Create client and chunker
    let client = Arc::from(create_llm_client(&config)?);
    let chunker = LLMConceptChunker::new(client);

    // Use the chunker
    let text = "Long document content here...";
    let chunks = chunker.chunk(text, 256).await;

    for (i, chunk) in chunks.iter().enumerate() {
        println!("Chunk {}: {} chars", i, chunk.len());
    }

    Ok(())
}
```

## How It Works

### LLM Concept Chunking

1. Sends text to LLM with instructions to identify semantic boundaries
2. LLM returns JSON array of semantically coherent chunks
3. Parses response and returns chunks
4. Falls back to word chunking on failure

**Prompt Template:**

```
You are a text segmentation assistant. Divide the following text
into logical chunks based on semantic concepts and topic boundaries.

Rules:
1. Each chunk should contain a complete concept or topic
2. Target approximately {size} words per chunk
3. Return ONLY a JSON array of strings
4. Preserve the original text exactly
```

### LLM Introspection Chunking

1. **Analysis Step**: LLM analyzes document structure and identifies topics
2. **Chunking Step**: Uses analysis to create optimized chunks
3. Falls back to word chunking on failure

This produces higher quality chunks but requires 2 LLM calls.

## Performance Considerations

| Strategy | LLM Calls | Speed | Quality |
|----------|-----------|-------|---------|
| Word | 0 | Fastest | Basic |
| LLM Concept | 1 | Medium | Good |
| LLM Introspection | 2 | Slowest | Best |

### Cost Estimation

For Ollama (free, local):

- No API costs
- CPU/GPU usage during processing

For OpenAI (gpt-4o-mini):

- ~$0.15 per 1M input tokens
- ~$0.60 per 1M output tokens

### Recommendations

| Use Case | Recommended |
|----------|-------------|
| Development/Testing | Ollama |
| Production (cost-sensitive) | Word chunking |
| Production (quality-focused) | OpenAI/Anthropic |
| Batch processing | Word chunking + periodic LLM |

## Troubleshooting

### LLM Chunkers Not Available

Check if LLM is configured:

```bash
curl http://localhost:8081/v1/params
```

If `llm-concept` and `llm-introspection` are listed, LLM is configured.

### Ollama Connection Failed

```bash
# Check Ollama is running
curl http://localhost:11434/api/tags

# If not running, start it
ollama serve
```

### Slow Response Times

- Use smaller models (e.g., `llama3` instead of `llama3:70b`)
- Increase timeout: `LLM_TIMEOUT=120`
- Consider using word chunking for large batches

### Falling Back to Word Chunking

LLM chunkers fall back when:

- LLM API is unavailable
- Response parsing fails
- Timeout occurs

Check logs for error messages:

```bash
RUST_LOG=info embedcache
```

## Best Practices

1. **Start with Ollama** for development to avoid API costs
2. **Use word chunking** for large-scale batch processing
3. **Reserve LLM chunking** for quality-critical content
4. **Monitor costs** when using paid APIs
5. **Set appropriate timeouts** based on your LLM performance
