# Running as a Service

This guide covers how to run EmbedCache as a REST API service.

## Starting the Service

### Basic Start

```bash
embedcache
```

This starts the service with default configuration on `127.0.0.1:8081`.

### With Configuration File

```bash
# Create .env file first
cat > .env << EOF
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
DB_PATH=/var/lib/embedcache/cache.db
ENABLED_MODELS=BGESmallENV15,AllMiniLML6V2
EOF

# Start the service
embedcache
```

### With Environment Variables

```bash
SERVER_HOST=0.0.0.0 SERVER_PORT=8080 embedcache
```

## API Endpoints

### Generate Embeddings

**POST** `/v1/embed`

Generate embeddings for a list of text strings.

```bash
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{
    "text": ["Hello, world!", "Another text to embed."],
    "config": {
      "chunking_type": "words",
      "chunking_size": 512,
      "embedding_model": "BGESmallENV15"
    }
  }'
```

**Response:**

```json
[
  [0.123, -0.456, 0.789, ...],
  [0.234, -0.567, 0.890, ...]
]
```

### Process URL

**POST** `/v1/process`

Fetch content from a URL, chunk it, and generate embeddings.

```bash
curl -X POST http://localhost:8081/v1/process \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com/article",
    "config": {
      "chunking_type": "words",
      "chunking_size": 256,
      "embedding_model": "BGESmallENV15"
    }
  }'
```

**Response:**

```json
{
  "url": "https://example.com/article",
  "config": {
    "chunking_type": "words",
    "chunking_size": 256,
    "embedding_model": "BGESmallENV15"
  },
  "chunks": {
    "0": "First chunk of text...",
    "1": "Second chunk of text..."
  },
  "embeddings": {
    "0": [0.123, -0.456, ...],
    "1": [0.234, -0.567, ...]
  },
  "error": null
}
```

### List Supported Features

**GET** `/v1/params`

Get a list of supported chunking types and embedding models.

```bash
curl http://localhost:8081/v1/params
```

**Response:**

```json
{
  "chunking_types": ["words", "llm-concept", "llm-introspection"],
  "embedding_models": [
    "AllMiniLML6V2",
    "BGESmallENV15",
    "BGEBaseENV15",
    ...
  ]
}
```

## Health Monitoring

Check if the service is running by making a request to the params endpoint:

```bash
curl -f http://localhost:8081/v1/params && echo "Service is healthy"
```

## Logging

EmbedCache uses Actix's built-in logging. Logs are written to stdout. To increase verbosity:

```bash
RUST_LOG=info embedcache
```

Log levels: `error`, `warn`, `info`, `debug`, `trace`

## Graceful Shutdown

The service handles SIGTERM gracefully, completing in-flight requests before shutting down.

```bash
# Send SIGTERM
kill -TERM $(pgrep embedcache)
```
