# Common Issues

Solutions to frequently encountered problems.

## Startup Issues

### Service Won't Start

**Symptoms:** Service fails to start, exits immediately.

**Diagnosis:**
```bash
RUST_LOG=debug embedcache
```

**Common Causes:**

1. **Port already in use**
   ```bash
   lsof -i :8081
   # Change port in .env
   SERVER_PORT=8082
   ```

2. **Invalid model name**
   ```bash
   # Check ENABLED_MODELS in .env
   ENABLED_MODELS=BGESmallENV15  # Correct
   ENABLED_MODELS=bge-small     # Wrong
   ```

3. **Database path not writable**
   ```bash
   # Check permissions
   touch /path/to/cache.db
   ```

### Model Download Fails

**Symptoms:** First startup hangs or fails during model download.

**Solutions:**

1. Check internet connectivity
2. Try a different model
3. Manually pre-download:
   ```rust
   // Run this once to download
   use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
   TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))?;
   ```

## API Errors

### 400 Bad Request: Unsupported embedding model

**Cause:** Requested model not in ENABLED_MODELS.

**Solution:**
```bash
# Add model to .env
ENABLED_MODELS=AllMiniLML6V2,BGESmallENV15,YourModel
```

### 400 Bad Request: Unsupported chunking type

**Cause:** LLM chunking requested but LLM not configured.

**Solution:**
```bash
# Either configure LLM
LLM_PROVIDER=ollama
LLM_MODEL=llama3

# Or use word chunking
curl ... -d '{"config": {"chunking_type": "words"}}'
```

### 500 Internal Server Error

**Diagnosis:**
```bash
# Check logs for details
RUST_LOG=debug embedcache
```

**Common Causes:**
- Database write failure
- Model loading error
- URL fetch failure

## Performance Issues

### Slow First Request

**Cause:** Model loading on first use.

**Solution:** Pre-warm after startup:
```bash
curl -X POST http://localhost:8081/v1/embed \
  -H "Content-Type: application/json" \
  -d '{"text": ["warmup"]}'
```

### High Memory Usage

**Cause:** Multiple large models loaded.

**Solutions:**
1. Enable fewer models
2. Use smaller models
3. Use quantized models (`*Q` variants)

### Slow URL Processing

**Causes:**
- URL fetch slow
- Large content
- LLM chunking enabled

**Solutions:**
1. Use word chunking for speed
2. Reduce chunk size
3. Check network connectivity

## LLM Issues

### LLM Chunkers Not Available

**Symptoms:** Only "words" in chunking_types.

**Diagnosis:**
```bash
curl http://localhost:8081/v1/params
```

**Solution:** Configure LLM provider:
```bash
LLM_PROVIDER=ollama
LLM_MODEL=llama3
LLM_BASE_URL=http://localhost:11434
```

### LLM Chunking Falls Back to Words

**Causes:**
- LLM server not running
- Timeout exceeded
- Invalid response

**Diagnosis:**
```bash
# Check Ollama
curl http://localhost:11434/api/tags

# Check logs
RUST_LOG=debug embedcache
```

**Solutions:**
1. Start LLM server
2. Increase timeout: `LLM_TIMEOUT=120`
3. Check LLM server logs

### Ollama Connection Refused

**Symptoms:** "Connection refused" in logs.

**Solutions:**
```bash
# Start Ollama
ollama serve

# Check if running
curl http://localhost:11434/api/tags

# If in Docker, use host network
LLM_BASE_URL=http://host.docker.internal:11434
```

## Cache Issues

### Cache Not Working

**Diagnosis:**
```bash
# Check cache file exists
ls -la cache.db

# Check cache contents
sqlite3 cache.db "SELECT COUNT(*) FROM cache;"
```

**Solutions:**
1. Check DB_PATH is writable
2. Check disk space
3. Verify cache table exists

### Cache File Growing Large

**Solution:** Vacuum the database:
```bash
sqlite3 cache.db "VACUUM;"
```

### Clear Cache

```bash
# Delete all entries
sqlite3 cache.db "DELETE FROM cache;"

# Or delete the file (when service stopped)
rm cache.db
```

## Docker Issues

### Container Exits Immediately

**Diagnosis:**
```bash
docker logs embedcache
```

**Solutions:**
1. Check environment variables
2. Verify volume mounts
3. Check resource limits

### Can't Connect to Host Ollama

**Solution:**
```bash
# Use host.docker.internal on macOS/Windows
LLM_BASE_URL=http://host.docker.internal:11434

# Or use host network on Linux
docker run --network host embedcache
```

### Permission Denied for Volume

**Solution:**
```bash
# Fix ownership
sudo chown -R 1000:1000 /path/to/data

# Or run as root (not recommended)
docker run --user root embedcache
```

## Debug Mode

Enable detailed logging:

```bash
# All logs
RUST_LOG=debug embedcache

# Specific modules
RUST_LOG=embedcache=debug,actix_web=info embedcache
```
