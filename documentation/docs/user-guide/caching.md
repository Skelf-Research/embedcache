# Caching

EmbedCache uses SQLite to cache processed content, avoiding redundant embedding generation.

## How Caching Works

1. When a URL is processed, a hash is generated from:
   - The URL
   - Chunking type
   - Chunk size
   - Embedding model

2. EmbedCache checks if this hash exists in the cache

3. If found, cached results are returned immediately

4. If not found, content is processed and cached for future use

## Cache Benefits

- **Cost Savings** - Avoid redundant embedding API calls
- **Speed** - Instant responses for cached content
- **Consistency** - Same input always returns same output

## Cache Location

By default, the cache is stored in `cache.db` in the current directory. Configure a different location:

```bash
DB_PATH=/var/lib/embedcache/cache.db
```

## Journal Modes

SQLite supports different journal modes that affect performance and durability:

```bash
DB_JOURNAL_MODE=wal  # Default, recommended
```

| Mode | Description | Best For |
|------|-------------|----------|
| `wal` | Write-Ahead Logging | High concurrency |
| `truncate` | Truncate journal on commit | Single process |
| `persist` | Keep journal file | Slow file deletion |

## Cache Key Generation

The cache key is a SHA-256 hash of:

```rust
hash = SHA256(url + chunking_type + chunking_size + embedding_model)
```

This means:

- Same URL with different config = different cache entry
- Different URL with same config = different cache entry

## Checking Cache Status

The cache is transparent - if a result is cached, it's returned without indication. To verify caching is working:

```bash
# First request (cache miss - slower)
time curl -X POST http://localhost:8081/v1/process ...

# Second request (cache hit - faster)
time curl -X POST http://localhost:8081/v1/process ...
```

## Cache Management

### View Cache Size

```bash
ls -lh cache.db
```

### Clear Cache

```bash
# Stop the service first
rm cache.db

# Or use SQLite
sqlite3 cache.db "DELETE FROM cache;"
```

### Vacuum Database

```bash
sqlite3 cache.db "VACUUM;"
```

## Cache Schema

The cache uses a simple schema:

```sql
CREATE TABLE cache (
  hash TEXT PRIMARY KEY,
  content TEXT
);
```

- `hash` - SHA-256 hash of request parameters
- `content` - JSON-serialized ProcessedContent

## Programmatic Cache Access

```rust
use embedcache::{get_from_cache, cache_result, initialize_db_pool, ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::from_env()?;
    let pool = initialize_db_pool(&config).await?;

    // Check cache
    let hash = "your_hash_here".to_string();
    if let Some(cached) = get_from_cache(&pool, hash.clone()).await? {
        println!("Cache hit!");
    } else {
        println!("Cache miss");
    }

    Ok(())
}
```

## Cache Invalidation

EmbedCache doesn't automatically invalidate cache entries. To update cached content:

1. Delete the specific entry
2. Clear the entire cache
3. Use a different configuration (creates new cache key)

## Production Recommendations

1. **Persistent Storage** - Store cache on persistent disk
2. **Regular Backups** - Back up the cache database
3. **Monitor Size** - Watch cache growth over time
4. **Periodic Cleanup** - Remove old entries if needed

```bash
# Example: Delete entries older than 30 days
sqlite3 cache.db "DELETE FROM cache WHERE rowid IN (SELECT rowid FROM cache LIMIT 1000);"
```
