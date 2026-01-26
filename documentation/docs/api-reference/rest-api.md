# REST API Reference

Complete reference for the EmbedCache REST API.

## Base URL

```
http://localhost:8081
```

## Authentication

EmbedCache does not require authentication by default. For production deployments, use a reverse proxy to add authentication.

---

## POST /v1/embed

Generate embeddings for a list of text strings.

### Request

```http
POST /v1/embed
Content-Type: application/json
```

**Body:**

```json
{
  "text": ["string1", "string2", ...],
  "config": {
    "chunking_type": "words",
    "chunking_size": 512,
    "embedding_model": "BGESmallENV15"
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text` | array[string] | Yes | List of texts to embed |
| `config` | object | No | Processing configuration |
| `config.chunking_type` | string | No | Chunking strategy (default: "words") |
| `config.chunking_size` | integer | No | Chunk size in words (default: 512) |
| `config.embedding_model` | string | No | Model to use (default: "BGESmallENV15") |

### Response

**Success (200 OK):**

```json
[
  [0.123, -0.456, 0.789, ...],
  [0.234, -0.567, 0.890, ...]
]
```

Array of embedding vectors, one per input text.

**Error (400 Bad Request):**

```json
{
  "error": "Unsupported embedding model: InvalidModel"
}
```

### Example

```bash
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{
    "text": [
      "Machine learning is a subset of AI.",
      "Natural language processing enables text understanding."
    ],
    "config": {
      "embedding_model": "BGESmallENV15"
    }
  }'
```

---

## POST /v1/process

Fetch content from a URL, chunk it, and generate embeddings. Results are cached.

### Request

```http
POST /v1/process
Content-Type: application/json
```

**Body:**

```json
{
  "url": "https://example.com/article",
  "config": {
    "chunking_type": "words",
    "chunking_size": 256,
    "embedding_model": "BGESmallENV15"
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | string | Yes | URL to fetch and process |
| `config` | object | No | Processing configuration |

### Response

**Success (200 OK):**

```json
{
  "url": "https://example.com/article",
  "config": {
    "chunking_type": "words",
    "chunking_size": 256,
    "embedding_model": "BGESmallENV15"
  },
  "chunks": {
    "0": "First chunk of extracted text...",
    "1": "Second chunk of extracted text...",
    "2": "Third chunk of extracted text..."
  },
  "embeddings": {
    "0": [0.123, -0.456, ...],
    "1": [0.234, -0.567, ...],
    "2": [0.345, -0.678, ...]
  },
  "error": null
}
```

| Field | Type | Description |
|-------|------|-------------|
| `url` | string | Processed URL |
| `config` | object | Configuration used |
| `chunks` | object | Map of chunk index to text |
| `embeddings` | object | Map of chunk index to embedding |
| `error` | string\|null | Error message if processing failed |

**Scraping Failed:**

```json
{
  "url": "https://example.com/article",
  "config": {...},
  "chunks": {},
  "embeddings": {},
  "error": "Failed to scrape content"
}
```

### Example

```bash
curl -X POST http://localhost:8081/v1/process \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://en.wikipedia.org/wiki/Machine_learning",
    "config": {
      "chunking_type": "words",
      "chunking_size": 200,
      "embedding_model": "AllMiniLML6V2"
    }
  }'
```

---

## GET /v1/params

List supported chunking types and embedding models.

### Request

```http
GET /v1/params
```

### Response

**Success (200 OK):**

```json
{
  "chunking_types": [
    "words",
    "llm-concept",
    "llm-introspection"
  ],
  "embedding_models": [
    "AllMiniLML6V2",
    "AllMiniLML6V2Q",
    "AllMiniLML12V2",
    "AllMiniLML12V2Q",
    "BGEBaseENV15",
    "BGEBaseENV15Q",
    "BGELargeENV15",
    "BGELargeENV15Q",
    "BGESmallENV15",
    "BGESmallENV15Q",
    "NomicEmbedTextV1",
    "NomicEmbedTextV15",
    "NomicEmbedTextV15Q",
    "ParaphraseMLMiniLML12V2",
    "ParaphraseMLMiniLML12V2Q",
    "ParaphraseMLMpnetBaseV2",
    "BGESmallZHV15",
    "MultilingualE5Small",
    "MultilingualE5Base",
    "MultilingualE5Large",
    "MxbaiEmbedLargeV1",
    "MxbaiEmbedLargeV1Q"
  ]
}
```

### Example

```bash
curl http://localhost:8081/v1/params
```

---

## Error Responses

All endpoints may return error responses:

**400 Bad Request:**

```json
{
  "error": "Unsupported chunking type: invalid-type"
}
```

**500 Internal Server Error:**

```json
{
  "error": "Internal server error message"
}
```

---

## OpenAPI Specification

The full OpenAPI specification is available at:

```
GET /openapi.json
```

---

## Rate Limiting

EmbedCache does not implement rate limiting. For production use, configure rate limiting in your reverse proxy.

---

## CORS

CORS is not configured by default. Enable it via a reverse proxy if needed.
