# Performance Tuning

This guide covers optimization techniques for EmbedCache.

## Benchmarking

### Measure Response Times

```bash
# Single request
time curl -s -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{"text": ["Test text"], "config": {"embedding_model": "AllMiniLML6V2"}}' \
  > /dev/null

# Multiple requests
for i in {1..10}; do
  time curl -s -X POST http://localhost:8081/v1/embed \
    -H "Content-Type: application/json" \
    -d '{"text": ["Test text"], "config": {"embedding_model": "AllMiniLML6V2"}}' \
    > /dev/null
done
```

### Load Testing

```bash
# Using wrk
wrk -t4 -c100 -d30s -s post.lua http://localhost:8081/v1/embed

# post.lua
wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = '{"text": ["Test text"], "config": {"embedding_model": "AllMiniLML6V2"}}'
```

## Model Selection

### Speed vs Quality Trade-offs

| Model | Speed | Quality | Memory |
|-------|-------|---------|--------|
| `AllMiniLML6V2` | Fastest | Good | Low |
| `AllMiniLML6V2Q` | Fastest | Good | Lowest |
| `BGESmallENV15` | Fast | Better | Low |
| `BGEBaseENV15` | Medium | Best | Medium |
| `BGELargeENV15` | Slow | Highest | High |

### Model Loading

Enable only needed models to reduce memory:

```bash
# .env - Only load what you need
ENABLED_MODELS=AllMiniLML6V2,BGESmallENV15
```

## Chunking Strategy

### Strategy Comparison

| Strategy | Speed | Best For |
|----------|-------|----------|
| `words` | Fastest | Batch processing |
| `llm-concept` | Slow | Quality-focused |
| `llm-introspection` | Slowest | Document analysis |

### Optimal Chunk Sizes

| Content Length | Recommended Size |
|----------------|-----------------|
| < 500 words | No chunking needed |
| 500-2000 words | 256-512 |
| 2000-10000 words | 512-1024 |
| > 10000 words | 512-1024 |

## Database Optimization

### Journal Mode

```bash
# WAL mode for high concurrency (default)
DB_JOURNAL_MODE=wal

# Truncate for single-process usage
DB_JOURNAL_MODE=truncate
```

### Cache Maintenance

```bash
# Vacuum to reclaim space
sqlite3 cache.db "VACUUM;"

# Analyze for query optimization
sqlite3 cache.db "ANALYZE;"
```

### Cache Hit Monitoring

```bash
# Check cache size
sqlite3 cache.db "SELECT COUNT(*) FROM cache;"

# Estimate hit rate (manual tracking needed)
```

## Memory Optimization

### Limit Concurrent Requests

Use a reverse proxy to limit concurrency:

```nginx
upstream embedcache {
    server 127.0.0.1:8081;
    keepalive 32;
}

server {
    location / {
        limit_req zone=embedcache burst=20;
        proxy_pass http://embedcache;
    }
}
```

### Model Memory Usage

| Model Type | Approximate RAM |
|------------|-----------------|
| Small (384 dim) | ~200MB |
| Base (768 dim) | ~400MB |
| Large (1024 dim) | ~800MB |

## Batch Processing

### Optimal Batch Sizes

```rust
// Process in batches for large inputs
let batch_size = 32; // Adjust based on memory

for chunk in texts.chunks(batch_size) {
    let embeddings = embedder.embed(chunk).await?;
    // Process embeddings...
}
```

### Parallel Processing

```rust
use futures::future::join_all;

// Process multiple batches in parallel
let futures: Vec<_> = batches
    .iter()
    .map(|batch| embedder.embed(batch))
    .collect();

let results = join_all(futures).await;
```

## Production Recommendations

### Hardware

| Component | Recommendation |
|-----------|---------------|
| CPU | 4+ cores |
| RAM | 4GB+ (8GB recommended) |
| Storage | SSD for database |

### Configuration

```bash
# Production .env
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
DB_PATH=/var/lib/embedcache/cache.db
DB_JOURNAL_MODE=wal
ENABLED_MODELS=BGESmallENV15
RUST_LOG=warn
```

### Monitoring

Monitor these metrics:

- Response time percentiles (p50, p95, p99)
- Request throughput
- Memory usage
- Cache hit rate
- Error rate

### Health Checks

```bash
# Simple health check
curl -f http://localhost:8081/v1/params || exit 1
```

## Troubleshooting Performance Issues

### Slow First Request

First request loads the model. This is normal.

```bash
# Pre-warm by making a request after startup
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{"text": ["warmup"]}' > /dev/null
```

### High Memory Usage

1. Reduce number of enabled models
2. Use quantized models (`*Q` variants)
3. Implement request queuing

### Slow Database

1. Enable WAL mode
2. Run VACUUM periodically
3. Use SSD storage

### LLM Chunking Slow

1. Use local LLM (Ollama)
2. Use smaller models
3. Increase timeout
4. Fall back to word chunking
