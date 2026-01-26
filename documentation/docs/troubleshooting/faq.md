# Frequently Asked Questions

## General

### What is EmbedCache?

EmbedCache is a high-performance Rust library and REST API service for generating text embeddings with intelligent caching.

### What embedding models are supported?

EmbedCache supports 22+ models through FastEmbed, including:
- AllMiniLM series
- BGE series (Small, Base, Large)
- Nomic Embed series
- Multilingual E5 series
- Paraphrase series
- MxbaiEmbed series

See [Embedding Models](../user-guide/embedding-models.md) for the complete list.

### Is EmbedCache free to use?

Yes, EmbedCache is open source under the GPL-3.0 license. The embedding models are also free and run locally.

### Do I need an API key?

No, for basic usage. Embedding generation happens locally using FastEmbed.

You only need API keys if you want to use LLM-based chunking with OpenAI or Anthropic.

## Usage

### How do I choose the right model?

| Need | Recommended Model |
|------|-------------------|
| Fast, general purpose | `AllMiniLML6V2` |
| Better quality | `BGESmallENV15` |
| Highest quality | `BGELargeENV15` |
| Multiple languages | `MultilingualE5Base` |
| Low memory | `AllMiniLML6V2Q` |

### What's the difference between /v1/embed and /v1/process?

- `/v1/embed` - Generates embeddings for text you provide
- `/v1/process` - Fetches URL content, chunks it, and generates embeddings

### Are results cached?

Only `/v1/process` results are cached. `/v1/embed` does not cache because the same text should produce the same embeddings.

### What chunk size should I use?

Start with 256-512 words. Smaller chunks provide more precise retrieval, larger chunks provide more context.

## LLM Chunking

### Do I need an LLM for EmbedCache?

No, EmbedCache works without LLM. Word-based chunking is always available.

LLM chunking is optional and provides higher quality semantic chunking.

### Which LLM provider should I use?

| Context | Recommended |
|---------|-------------|
| Development | Ollama (free, local) |
| Production (budget) | Ollama |
| Production (quality) | OpenAI or Anthropic |

### Why does LLM chunking fall back to word chunking?

This happens when:
- LLM server is unavailable
- Request times out
- Response can't be parsed

Check logs for specific errors.

## Performance

### How fast is EmbedCache?

Typical response times:
- Embed (cached model): 10-100ms per text
- Process (cache hit): < 10ms
- Process (cache miss): 1-5 seconds (depends on URL and content size)

### How much memory does it use?

Memory usage depends on loaded models:
- Small models (~384 dim): ~200MB each
- Base models (~768 dim): ~400MB each
- Large models (~1024 dim): ~800MB each

### Can I run multiple instances?

Yes, for read scaling. However, each instance has its own cache unless you share the SQLite database (with proper locking).

## Deployment

### How do I deploy to production?

1. Use a reverse proxy (Nginx, Caddy) for TLS
2. Configure authentication in the proxy
3. Set up monitoring
4. Use persistent storage for the cache database

See [Deployment](../deployment/index.md) for detailed guides.

### Does EmbedCache support horizontal scaling?

Limited. The SQLite cache doesn't support distributed access. For horizontal scaling:
- Use a shared cache layer (Redis)
- Implement your own caching
- Use sticky sessions

### What are the system requirements?

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| RAM | 2GB | 4GB+ |
| CPU | 2 cores | 4+ cores |
| Disk | 500MB | SSD recommended |

## Troubleshooting

### Why is the first request slow?

The embedding model is loaded on first use. Subsequent requests are faster.

### Why am I getting "Unsupported embedding model"?

The model isn't in your ENABLED_MODELS configuration. Add it to your .env file.

### How do I reset the cache?

```bash
# Stop the service, then:
rm cache.db

# Or:
sqlite3 cache.db "DELETE FROM cache;"
```

## Development

### Can I add custom chunking strategies?

Yes! Implement the `ContentChunker` trait. See [Custom Chunkers](../advanced/custom-chunkers.md).

### Can I use different embedding providers?

Yes! Implement the `Embedder` trait. See [Custom Embedders](../advanced/custom-embedders.md).

### How do I contribute?

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

See the [Contributing Guidelines](https://github.com/skelfresearch/embedcache/blob/main/CONTRIBUTING.md).
