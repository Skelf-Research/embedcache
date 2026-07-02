# EmbedCache

**Stop recomputing embeddings. Start shipping faster.**

[![Crates.io](https://img.shields.io/crates/v/embedcache.svg)](https://crates.io/crates/embedcache)
[![Documentation](https://img.shields.io/badge/docs-skelfresearch.com-blue.svg)](https://docs.skelfresearch.com/embedcache)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](https://opensource.org/licenses/GPL-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

<p align="center">
  <a href="https://embedcache.skelfresearch.com"><b>Website</b></a> •
  <a href="https://docs.skelfresearch.com/embedcache">Documentation</a> •
  <a href="https://skelfresearch.com">Skelf Research</a>
</p>

EmbedCache is a Rust library and REST API that generates text embeddings locally and caches the results. No external API calls, no per-token billing, no rate limits. Just fast, local embeddings with 22+ models.

---

## Why EmbedCache?

Building RAG apps, semantic search, or anything with embeddings? You've probably hit these problems:

- **Recomputing the same embeddings** every time you restart your app
- **Paying for API calls** to embed text you've already processed
- **Waiting on rate limits** when you need to embed thousands of documents
- **Vendor lock-in** to a specific embedding provider

EmbedCache fixes all of this. Embeddings are generated locally using [FastEmbed](https://github.com/qdrant/fastembed) and cached in SQLite. Process a URL once, get instant results forever.

## Features

- **22+ embedding models** - BGE, MiniLM, Nomic, E5 multilingual, and more
- **Local inference** - No API keys, no costs, no rate limits
- **Automatic caching** - SQLite-backed, survives restarts
- **LLM-powered chunking** - Optional semantic chunking via Ollama/OpenAI
- **Dual interface** - Use as a Rust library or REST API
- **Built-in docs** - Swagger, ReDoc, RapiDoc, Scalar

## Quick Start

### As a Service

```bash
cargo install embedcache
embedcache
```

```bash
# Generate embeddings
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{"text": ["Hello world", "Semantic search is cool"]}'

# Process a URL (fetches, chunks, embeds, caches)
curl -X POST http://localhost:8081/v1/process \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com/article"}'
```

### As a Library

```toml
[dependencies]
embedcache = "0.1"
tokio = { version = "1", features = ["full"] }
```

```rust
use embedcache::{FastEmbedder, Embedder};
use fastembed::{InitOptions, EmbeddingModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let embedder = FastEmbedder {
        options: InitOptions::new(EmbeddingModel::BGESmallENV15),
    };

    let texts = vec![
        "First document to embed".to_string(),
        "Second document to embed".to_string(),
    ];

    let embeddings = embedder.embed(&texts).await?;
    println!("Generated {} embeddings of {} dimensions",
             embeddings.len(), embeddings[0].len());
    Ok(())
}
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/v1/embed` | POST | Generate embeddings for text array |
| `/v1/process` | POST | Fetch URL, chunk, embed, and cache |
| `/v1/params` | GET | List available models and chunkers |

Interactive docs at `/swagger`, `/redoc`, `/rapidoc`, or `/scalar`.

## Configuration

Create a `.env` file or set environment variables:

```bash
SERVER_HOST=127.0.0.1
SERVER_PORT=8081
DB_PATH=cache.db
ENABLED_MODELS=BGESmallENV15,AllMiniLML6V2

# Optional: LLM-powered chunking
LLM_PROVIDER=ollama
LLM_MODEL=llama3
LLM_BASE_URL=http://localhost:11434
```

## Supported Models

| Model | Dimensions | Use Case |
|-------|------------|----------|
| `AllMiniLML6V2` | 384 | Fast, general purpose |
| `BGESmallENV15` | 384 | Best quality/speed balance |
| `BGEBaseENV15` | 768 | Higher quality |
| `BGELargeENV15` | 1024 | Highest quality |
| `MultilingualE5Base` | 768 | 100+ languages |

[See all 22+ models →](https://docs.skelfresearch.com/embedcache/user-guide/embedding-models/)

## Chunking Strategies

| Strategy | Description |
|----------|-------------|
| `words` | Split by whitespace (fast, always available) |
| `llm-concept` | LLM identifies semantic boundaries |
| `llm-introspection` | LLM analyzes then chunks (highest quality) |

## Custom Chunkers

Implement the `ContentChunker` trait:

```rust
use embedcache::ContentChunker;
use async_trait::async_trait;

struct SentenceChunker;

#[async_trait]
impl ContentChunker for SentenceChunker {
    async fn chunk(&self, content: &str, _size: usize) -> Vec<String> {
        content.split(". ")
            .map(|s| s.to_string())
            .collect()
    }

    fn name(&self) -> &str { "sentences" }
}
```

## Performance

- **First request**: ~100-500ms (model loading)
- **Subsequent requests**: ~10-50ms per text
- **Cache hits**: <5ms

Memory usage depends on enabled models (~200MB-800MB each).

## Documentation

- [Full Documentation](https://docs.skelfresearch.com/embedcache/)
- [API Reference](https://docs.skelfresearch.com/embedcache/api-reference/rest-api/)
- [Deployment Guide](https://docs.skelfresearch.com/embedcache/deployment/)

Build docs locally:

```bash
cd documentation
pip install -r requirements.txt
mkdocs serve
```

## Project Structure

```
src/
├── chunking/          # Text chunking (word, LLM-based)
├── embedding/         # Embedding generation (FastEmbed)
├── handlers/          # HTTP endpoints
├── cache/             # SQLite caching
├── models/            # Data types
└── utils/             # Hash generation, URL fetching
```

## Contributing

```bash
git clone https://github.com/skelfresearch/embedcache
cd embedcache
cargo build
cargo test
```

PRs welcome. Please open an issue first for major changes.

## License

GPL-3.0. See [LICENSE](LICENSE).

## Links

- [GitHub](https://github.com/skelfresearch/embedcache)
- [Crates.io](https://crates.io/crates/embedcache)
- [Documentation](https://docs.skelfresearch.com/embedcache/)

---

Built by [Skelf Research](https://skelfresearch.com) with [FastEmbed](https://github.com/qdrant/fastembed) and [Actix-web](https://actix.rs/).

---

## Part of Skelf Research

`embedcache` is built by **[Skelf Research](https://skelfresearch.com)** — an independent UK AI research lab publishing production-grade open-source projects.

🌐 [Website](https://embedcache.skelfresearch.com) · 📚 [Documentation](https://docs.skelfresearch.com/embedcache) · 🔬 [All projects](https://skelfresearch.com/projects) · 🤗 [Hugging Face](https://huggingface.co/skelfresearch)

**Related projects:** [memista](https://memista.skelfresearch.com) (vector search for Rust) · [polymathy](https://polymathy.skelfresearch.com) (answer-engine service) · [slorg](https://slorg.skelfresearch.com) (search that thinks first)

<sub>Released under MIT / Apache-2.0. © Skelf Research Limited.</sub>
