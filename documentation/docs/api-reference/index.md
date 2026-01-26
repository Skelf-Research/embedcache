# API Reference

This section provides detailed API documentation for EmbedCache.

## Overview

EmbedCache provides:

- **REST API** - HTTP endpoints for embedding generation and URL processing
- **Rust API** - Library interface for direct integration

## REST API

The REST API is available when running EmbedCache as a service.

**Base URL:** `http://localhost:8081` (default)

### Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/v1/embed` | Generate embeddings for text |
| POST | `/v1/process` | Process URL and generate embeddings |
| GET | `/v1/params` | List supported features |

### Interactive Documentation

When the service is running, interactive API documentation is available:

- [Swagger UI](/swagger) - Interactive API explorer
- [ReDoc](/redoc) - Clean API documentation
- [RapiDoc](/rapidoc) - Modern API documentation
- [Scalar](/scalar) - Beautiful API reference
- [OpenAPI JSON](/openapi.json) - Raw OpenAPI specification

## Rust API

The library API provides direct access to EmbedCache functionality.

### Main Types

| Type | Description |
|------|-------------|
| `FastEmbedder` | Primary embedding generator |
| `WordChunker` | Word-based text chunking |
| `LLMConceptChunker` | LLM concept-based chunking |
| `LLMIntrospectionChunker` | LLM introspection chunking |
| `AppState` | Application state container |
| `Config` | Request configuration |

### Main Functions

| Function | Description |
|----------|-------------|
| `initialize_db_pool` | Create database connection pool |
| `initialize_models` | Load embedding models |
| `initialize_chunkers` | Create chunker instances |
| `get_embedding_model` | Get model by name |
| `generate_hash` | Generate cache hash |
| `fetch_content` | Fetch URL content |

## Response Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 400 | Bad Request (invalid model/chunker) |
| 500 | Internal Server Error |
