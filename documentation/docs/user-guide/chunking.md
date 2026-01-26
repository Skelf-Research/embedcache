# Chunking Strategies

EmbedCache provides multiple text chunking strategies to break down documents into smaller pieces for embedding generation.

## Why Chunking Matters

Embedding models have token limits and work best with focused, coherent text segments. Chunking strategies help:

- **Stay within model limits** - Avoid truncation
- **Improve embedding quality** - More focused embeddings
- **Enable semantic search** - Find specific passages
- **Optimize storage** - Index at appropriate granularity

## Available Strategies

### Word Chunking

**Type:** `words`

The simplest strategy - splits text by whitespace into fixed-size word chunks.

```bash
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{
    "text": ["Your long text here..."],
    "config": {
      "chunking_type": "words",
      "chunking_size": 512
    }
  }'
```

**Characteristics:**

- Fast and deterministic
- May split mid-sentence or mid-concept
- Good for general-purpose use
- Always available

### LLM Concept Chunking

**Type:** `llm-concept`

Uses an LLM to identify semantic concept boundaries in the text.

```bash
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{
    "text": ["Your long text here..."],
    "config": {
      "chunking_type": "llm-concept",
      "chunking_size": 256
    }
  }'
```

**Characteristics:**

- Semantically coherent chunks
- Respects topic boundaries
- Slower than word chunking
- Requires LLM configuration
- Falls back to word chunking on failure

### LLM Introspection Chunking

**Type:** `llm-introspection`

Uses a two-step LLM process: first analyzes document structure, then creates optimized chunks.

```bash
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{
    "text": ["Your long text here..."],
    "config": {
      "chunking_type": "llm-introspection",
      "chunking_size": 256
    }
  }'
```

**Characteristics:**

- Best semantic quality
- Document-aware chunking
- Slowest option (2 LLM calls)
- Requires LLM configuration
- Falls back to word chunking on failure

## Choosing a Strategy

| Use Case | Recommended Strategy |
|----------|---------------------|
| High throughput processing | `words` |
| Semantic search quality | `llm-concept` |
| Document analysis | `llm-introspection` |
| Limited LLM budget | `words` |
| Best retrieval accuracy | `llm-introspection` |

## Chunk Size Guidelines

| Content Type | Recommended Size |
|--------------|-----------------|
| Short documents | 128-256 words |
| Articles | 256-512 words |
| Long documents | 512-1024 words |
| Technical docs | 256-512 words |

!!! tip "Finding Optimal Size"
    Start with 256-512 words and adjust based on your search results quality. Smaller chunks provide more precise retrieval, larger chunks provide more context.

## Configuring LLM Chunking

To use LLM-based chunking, configure an LLM provider:

```bash
# In .env file
LLM_PROVIDER=ollama
LLM_MODEL=llama3
LLM_BASE_URL=http://localhost:11434
```

See [LLM Chunking](../advanced/llm-chunking.md) for detailed setup.

## Custom Chunking

You can implement custom chunking strategies. See [Custom Chunkers](../advanced/custom-chunkers.md).

## Example: Comparing Strategies

```python
import requests

text = """
Machine learning is a subset of artificial intelligence that enables
computers to learn from data. Deep learning, a type of machine learning,
uses neural networks with many layers. Natural language processing (NLP)
allows computers to understand human language.
"""

for strategy in ["words", "llm-concept"]:
    response = requests.post(
        "http://localhost:8081/v1/embed",
        json={
            "text": [text],
            "config": {
                "chunking_type": strategy,
                "chunking_size": 20
            }
        }
    )
    print(f"{strategy}: {len(response.json())} embeddings")
```
