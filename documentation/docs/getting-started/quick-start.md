# Quick Start

Get EmbedCache running in 5 minutes.

## Starting the Service

1. Create a configuration file (optional):

```bash
# Copy the sample configuration
cp sample.env .env

# Edit as needed
nano .env
```

2. Start the server:

```bash
embedcache
```

You should see:

```
LLM not configured. Only word chunking available.
Starting server at 127.0.0.1:8081
```

## Making Your First API Call

### Generate Embeddings

```bash
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{
    "text": ["Hello, world!", "This is a test."],
    "config": {
      "chunking_type": "words",
      "chunking_size": 512,
      "embedding_model": "AllMiniLML6V2"
    }
  }'
```

### Process a URL

```bash
curl -X POST http://localhost:8081/v1/process \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "config": {
      "chunking_type": "words",
      "chunking_size": 256,
      "embedding_model": "AllMiniLML6V2"
    }
  }'
```

### List Supported Features

```bash
curl http://localhost:8081/v1/params
```

## Using as a Library

```rust
use embedcache::{FastEmbedder, Embedder};
use fastembed::{InitOptions, EmbeddingModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an embedder
    let embedder = FastEmbedder {
        options: InitOptions::new(EmbeddingModel::BGESmallENV15),
    };

    // Texts to embed
    let texts = vec![
        "Machine learning is fascinating.".to_string(),
        "Natural language processing enables computers to understand text.".to_string(),
    ];

    // Generate embeddings
    let embeddings = embedder.embed(&texts).await?;

    // Use the embeddings
    for (i, embedding) in embeddings.iter().enumerate() {
        println!("Text {}: {} dimensions", i, embedding.len());
    }

    Ok(())
}
```

## API Documentation

EmbedCache comes with built-in API documentation. Once the server is running, visit:

- **Swagger UI**: [http://localhost:8081/swagger](http://localhost:8081/swagger)
- **ReDoc**: [http://localhost:8081/redoc](http://localhost:8081/redoc)
- **RapiDoc**: [http://localhost:8081/rapidoc](http://localhost:8081/rapidoc)
- **Scalar**: [http://localhost:8081/scalar](http://localhost:8081/scalar)
- **OpenAPI JSON**: [http://localhost:8081/openapi.json](http://localhost:8081/openapi.json)

## Next Steps

- [Configuration](configuration.md) - Customize EmbedCache for your needs
- [Chunking Strategies](../user-guide/chunking.md) - Learn about different chunking options
- [Embedding Models](../user-guide/embedding-models.md) - Explore available models
