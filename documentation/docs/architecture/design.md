# Design Decisions

This document explains the key design decisions in EmbedCache.

## Dual Interface (Library + Service)

**Decision:** EmbedCache works both as a Rust library and a REST API service.

**Rationale:**
- Library mode enables direct integration without network overhead
- Service mode provides language-agnostic access
- Same core code serves both use cases
- Easy to start with service, migrate to library for performance

## Trait-Based Extensibility

**Decision:** Core functionality uses traits (`ContentChunker`, `Embedder`).

**Rationale:**
- Easy to add custom implementations
- Test with mocks and stubs
- Swap implementations without changing callers
- Future-proof for new embedding providers

## SQLite for Caching

**Decision:** Use SQLite for caching processed content.

**Rationale:**
- Zero configuration database
- Single file deployment
- ACID compliance
- Good performance for read-heavy workloads
- WAL mode supports concurrent reads

**Trade-offs:**
- Not suitable for distributed caching
- Write throughput limited
- Not designed for multi-server deployments

## Hash-Based Cache Keys

**Decision:** Cache keys are SHA-256 hashes of URL + config.

**Rationale:**
- Deterministic: same input = same key
- Fixed length regardless of URL length
- Collision-resistant
- Config changes create new cache entries

## FastEmbed for Embedding

**Decision:** Use FastEmbed library for embedding generation.

**Rationale:**
- Local inference (no API costs)
- Multiple model support
- Rust-native library
- Good performance

**Trade-offs:**
- Memory usage for models
- Initial download required
- Limited to FastEmbed-supported models

## Optional LLM Chunking

**Decision:** LLM-based chunking is optional, falls back to word chunking.

**Rationale:**
- Works without LLM infrastructure
- Progressive enhancement
- Graceful degradation on failure
- Users choose cost/quality trade-off

## Async Architecture

**Decision:** Fully async with Tokio runtime.

**Rationale:**
- Efficient handling of I/O-bound operations
- Better resource utilization
- Scales with concurrent requests
- Required for HTTP service

## Configuration via Environment

**Decision:** Configure via environment variables and `.env` file.

**Rationale:**
- 12-factor app compliance
- Easy Docker/container deployment
- No config file parsing complexity
- Works with most deployment systems

## Modular Source Structure

**Decision:** Split code into focused modules.

**Rationale:**
- Clear separation of concerns
- Easier to understand and maintain
- Parallel development possible
- Focused testing

## No Authentication Built-in

**Decision:** Authentication handled by reverse proxy.

**Rationale:**
- Simpler core service
- Flexible auth options via proxy
- Separation of concerns
- Production deployments use proxies anyway

## Error Handling

**Decision:** Use `anyhow` for library, `actix_web::Error` for handlers.

**Rationale:**
- `anyhow` provides rich context
- Handler errors map to HTTP responses
- Consistent error handling pattern
- Easy error propagation with `?`

## No Built-in Rate Limiting

**Decision:** Rate limiting handled by reverse proxy.

**Rationale:**
- More flexible configuration
- Proxy already handles this well
- Simpler service code
- Standard deployment pattern

## Chunking Size in Words

**Decision:** Chunk size specified in words, not tokens.

**Rationale:**
- More intuitive for users
- Consistent across models
- Easier to reason about
- Approximate token counts vary by model
