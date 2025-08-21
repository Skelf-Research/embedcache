# EmbedCache: High-performance Rust library for text embedding with caching

EmbedCache is a high-performance Rust library for generating text embeddings with built-in caching capabilities. It provides both a library interface for direct integration into your Rust projects and a REST API service for standalone deployment.

The library is designed to make it easy to generate high-quality embeddings for text data while minimizing computational costs through intelligent caching. Whether you're building a search engine, recommendation system, or any application that requires text embeddings, EmbedCache can help you do it efficiently.

## What does EmbedCache do?

EmbedCache solves the common problem of repeatedly computing embeddings for the same text data. It provides:

1. **Text Embedding Generation**: Generate high-quality vector embeddings for text using state-of-the-art models
2. **Intelligent Caching**: Automatically cache embeddings to avoid recomputation
3. **Flexible Text Chunking**: Break large texts into smaller chunks for processing
4. **Multiple Model Support**: Use from a wide variety of pre-trained embedding models
5. **REST API Service**: Deploy as a standalone service for centralized embedding generation

## Use Cases

- **Search Engines**: Generate embeddings for documents and queries to enable semantic search
- **Recommendation Systems**: Create user and item embeddings for personalized recommendations
- **Content Analysis**: Process large volumes of text for clustering, classification, or similarity analysis
- **Chatbots & AI Assistants**: Generate embeddings for context understanding and response generation
- **Plagiarism Detection**: Compare document embeddings to identify similar content
- **Content Recommendation**: Suggest similar articles, products, or content based on embeddings

## How is it used?

EmbedCache can be used in two primary ways:

1. **As a Library**: Integrate directly into your Rust applications for maximum performance and control
2. **As a Service**: Run the standalone REST API service and access it from any language or system

The library handles all the complexity of model management, text chunking, embedding generation, and caching, allowing you to focus on building your application.

## Features

- **Multiple Embedding Models**: Support for 20+ state-of-the-art embedding models including BGE, MiniLM, Nomic, and multilingual models
- **Modular Chunking Strategies**: Flexible text chunking with extensible trait-based architecture for custom implementations
- **Intelligent Caching**: SQLite-based caching with configurable journal modes to avoid recomputing embeddings
- **Dual Interface**: Use as a library in Rust applications or as a standalone REST API service
- **Asynchronous Operation**: Fully async/await compatible for high-performance applications
- **Built-in Documentation**: Automatic API documentation with Swagger UI, Redoc, and RapiDoc

## Quick Start

### Prerequisites

- Rust (latest stable version)
- SQLite

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
embedcache = "0.1.0"
```

Or to install the `embedcache` binary directly from crates.io:

```bash
cargo install embedcache
```

### Using as a Library

Here's a simple example of using EmbedCache as a library in your Rust project:

```rust
use embedcache::{FastEmbedder, Embedder, get_embedding_model};
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize a model
    let model = TextEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::BGESmallENV15,
        show_download_progress: true,
        ..Default::default()
    })?;

    // Create an embedder
    let embedder = FastEmbedder { options: InitOptions::new(EmbeddingModel::BGESmallENV15) };

    // Texts to embed
    let texts = vec![
        "This is an example sentence.".to_string(),
        "Another example sentence for embedding.".to_string(),
    ];

    // Generate embeddings
    let embeddings = embedder.embed(&texts).await?;

    println!("Generated {} embeddings", embeddings.len());
    for (i, embedding) in embeddings.iter().enumerate() {
        println!("Text {}: First 5 embedding values: {:?}", i, &embedding[..5.min(embedding.len())]);
    }

    Ok(())
}
```

### Running the embedcache Service

To run the standalone service:

1. Clone the repository:
```bash
git clone https://github.com/sokratis-xyz/embedcache.git
cd embedcache
```

2. Create a `.env` file with your configuration:
```bash
SERVER_HOST=127.0.0.1
SERVER_PORT=8081
DB_PATH=cache.db
DB_JOURNAL_MODE=wal
ENABLED_MODELS=BGESmallENV15,AllMiniLML6V2
```

3. Build and run:
```bash
cargo build --release
cargo run --release
```

The server will start at `http://127.0.0.1:8081` (or your configured host/port).

You can also run the installed binary directly:
```bash
embedcache
```

### Using as a Library

EmbedCache can also be used as a library in your own Rust projects. Add this to your `Cargo.toml`:

```toml
[dependencies]
embedcache = "0.1.0"  # Use the latest version from crates.io
```

Then you can use the library functions in your code:

```rust
use embedcache::{embed_text, Config};

// Use the functions as needed
```

## API Endpoints

When running the embedcache service, the following endpoints are available:

### `POST /v1/embed`
Generate embeddings for a list of text strings.

Request body:
```json
{
  "text": ["your text here", "another text"],
  "config": {
    "chunking_type": "words",
    "chunking_size": 512,
    "embedding_model": "BGESmallENV15"
  }
}
```

### `POST /v1/process`
Process a URL by extracting content, chunking, and generating embeddings.

Request body:
```json
{
  "url": "https://example.com",
  "config": {
    "chunking_type": "words",
    "chunking_size": 512,
    "embedding_model": "BGESmallENV15"
  }
}
```

### `GET /v1/params`
List supported chunking types and embedding models.

## Library Usage Examples

See the [examples](examples/) directory for more detailed usage examples:

1. [Simple Usage](examples/simple_usage.rs) - Basic library usage
2. [Library Usage](examples/library_usage.rs) - Complete example of using the library in your own project

## Configuration

When running as a service, embedcache can be configured through environment variables:

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| SERVER_HOST | 127.0.0.1 | Server host address |
| SERVER_PORT | 8081 | Server port |
| DB_PATH | cache.db | SQLite database path |
| DB_JOURNAL_MODE | wal | SQLite journal mode (wal/truncate/persist) |
| ENABLED_MODELS | AllMiniLML6V2 | Comma-separated list of enabled models |

## Supported Models

The service supports various embedding models including:
- BGE models (Small, Base, Large)
- AllMiniLM models
- Nomic Embed models
- Multilingual E5 models
- MxbaiEmbed models

For a complete list when running as a service, check the `/v1/params` endpoint.

When using as a library, you can use any model supported by the [fastembed](https://crates.io/crates/fastembed) crate.

## Documentation

### API Documentation

When running as a service, API documentation is available at:
- Swagger UI: `/swagger`
- Redoc: `/redoc`
- RapiDoc: `/rapidoc`
- OpenAPI JSON: `/openapi.json`

### Library Documentation

For detailed documentation on using EmbedCache as a library, you can generate the Rust documentation:

```bash
cargo doc --open
```

This will open the documentation in your browser, showing all public APIs and usage examples.

## Extending Chunking Strategies

EmbedCache provides a modular approach to text chunking through the `ContentChunker` trait. You can implement custom chunking strategies by implementing this trait:

```rust
use embedcache::{ContentChunker, AppState};
use async_trait::async_trait;

struct MyCustomChunker;

#[async_trait]
impl ContentChunker for MyCustomChunker {
    async fn chunk(&self, content: &str, size: usize) -> Vec<String> {
        // Your custom chunking logic here
        vec![content.to_string()] // Simplified example
    }
}

// Then register it with the AppState
// let mut chunkers = HashMap::new();
// chunkers.insert("my-custom-chunker".to_string(), Box::new(MyCustomChunker));
```

This allows you to easily extend the library with your own chunking algorithms while maintaining compatibility with the rest of the system.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📧 Contact

support@sokratis.xyz
