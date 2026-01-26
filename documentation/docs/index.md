# EmbedCache

**High-performance text embedding library and service with intelligent caching**

EmbedCache is a Rust library and REST API service that provides fast text embedding generation with built-in caching capabilities. It supports multiple state-of-the-art embedding models and offers flexible text chunking strategies, including LLM-powered intelligent chunking.

## Key Features

- **Multiple Embedding Models** - Support for 22+ models including BGE, MiniLM, Nomic, and Multilingual E5
- **Intelligent Chunking** - Word-based, LLM concept-based, and LLM introspection-based chunking strategies
- **Built-in Caching** - SQLite-based caching to avoid redundant embedding computations
- **Dual Interface** - Use as a Rust library or deploy as a REST API service
- **Extensible** - Easy to add custom chunking strategies and embedding providers
- **High Performance** - Async/await architecture with efficient batch processing

## Quick Start

### As a Service

```bash
# Install
cargo install embedcache

# Run
embedcache
```

### As a Library

```rust
use embedcache::{FastEmbedder, Embedder};
use fastembed::{InitOptions, EmbeddingModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let embedder = FastEmbedder {
        options: InitOptions::new(EmbeddingModel::BGESmallENV15),
    };

    let texts = vec![
        "Hello, world!".to_string(),
        "EmbedCache is awesome!".to_string(),
    ];

    let embeddings = embedder.embed(&texts).await?;
    println!("Generated {} embeddings", embeddings.len());
    Ok(())
}
```

## Use Cases

- **Semantic Search** - Generate embeddings for search indexing and retrieval
- **RAG Applications** - Process documents for retrieval-augmented generation
- **Text Similarity** - Compare text documents using embedding similarity
- **Content Deduplication** - Identify similar or duplicate content
- **Caching Layer** - Reduce embedding API costs with intelligent caching

## Getting Started

Ready to get started? Check out the [Installation Guide](getting-started/installation.md) or jump straight to the [Quick Start Tutorial](getting-started/quick-start.md).

## License

EmbedCache is released under the GPL-3.0 license. See the [LICENSE](https://github.com/skelfresearch/embedcache/blob/main/LICENSE) file for details.
