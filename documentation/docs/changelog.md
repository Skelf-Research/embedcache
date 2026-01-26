# Changelog

All notable changes to EmbedCache will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024

### Added

- Initial release
- REST API with three endpoints:
  - POST `/v1/embed` - Generate embeddings for text
  - POST `/v1/process` - Process URL and generate embeddings
  - GET `/v1/params` - List supported features
- Support for 22+ embedding models via FastEmbed:
  - AllMiniLM series
  - BGE series
  - Nomic series
  - Multilingual E5 series
  - Paraphrase series
  - MxbaiEmbed series
- Three chunking strategies:
  - Word-based chunking
  - LLM concept-based chunking
  - LLM introspection-based chunking
- SQLite-based caching for processed content
- LLM provider support:
  - Ollama
  - OpenAI
  - Anthropic
- Configuration via environment variables
- Built-in API documentation:
  - Swagger UI
  - ReDoc
  - RapiDoc
  - Scalar
- Modular architecture with extensible traits:
  - `ContentChunker` for custom chunking
  - `Embedder` for custom embedding
- Comprehensive MkDocs documentation

### Security

- No known security issues

---

## Future Plans

### Planned Features

- [ ] Redis cache backend option
- [ ] Batch processing API
- [ ] Async model loading
- [ ] Metrics endpoint (Prometheus)
- [ ] More embedding providers
- [ ] Sentence-based chunking
- [ ] Token-based chunk size

### Under Consideration

- Distributed cache support
- gRPC API
- WebSocket streaming
- Custom model loading
