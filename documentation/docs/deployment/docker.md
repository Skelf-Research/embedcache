# Docker Deployment

Deploy EmbedCache using Docker.

## Dockerfile

Create a `Dockerfile` in your project:

```dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source
COPY . .

# Build release binary
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/embedcache /app/embedcache

# Create data directory
RUN mkdir -p /data

# Environment defaults
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8081
ENV DB_PATH=/data/cache.db
ENV ENABLED_MODELS=AllMiniLML6V2

EXPOSE 8081

VOLUME ["/data"]

CMD ["/app/embedcache"]
```

## Building the Image

```bash
docker build -t embedcache:latest .
```

## Running

### Basic

```bash
docker run -p 8081:8081 embedcache:latest
```

### With Persistent Storage

```bash
docker run -p 8081:8081 \
  -v embedcache-data:/data \
  embedcache:latest
```

### With Custom Configuration

```bash
docker run -p 8081:8081 \
  -v embedcache-data:/data \
  -e ENABLED_MODELS=BGESmallENV15,AllMiniLML6V2 \
  -e LLM_PROVIDER=ollama \
  -e LLM_BASE_URL=http://host.docker.internal:11434 \
  embedcache:latest
```

## Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  embedcache:
    build: .
    ports:
      - "8081:8081"
    volumes:
      - embedcache-data:/data
    environment:
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8081
      - DB_PATH=/data/cache.db
      - ENABLED_MODELS=BGESmallENV15,AllMiniLML6V2
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8081/v1/params"]
      interval: 30s
      timeout: 10s
      retries: 3

  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
    volumes:
      - ollama-data:/root/.ollama
    restart: unless-stopped

volumes:
  embedcache-data:
  ollama-data:
```

### With Ollama for LLM Chunking

```yaml
version: '3.8'

services:
  embedcache:
    build: .
    ports:
      - "8081:8081"
    volumes:
      - embedcache-data:/data
    environment:
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8081
      - DB_PATH=/data/cache.db
      - ENABLED_MODELS=BGESmallENV15
      - LLM_PROVIDER=ollama
      - LLM_MODEL=llama3
      - LLM_BASE_URL=http://ollama:11434
    depends_on:
      - ollama
    restart: unless-stopped

  ollama:
    image: ollama/ollama:latest
    volumes:
      - ollama-data:/root/.ollama
    restart: unless-stopped

volumes:
  embedcache-data:
  ollama-data:
```

## Running with Docker Compose

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f embedcache

# Stop services
docker-compose down
```

## Resource Limits

```yaml
services:
  embedcache:
    deploy:
      resources:
        limits:
          memory: 4G
          cpus: '2'
        reservations:
          memory: 2G
          cpus: '1'
```

## Health Checks

The Dockerfile includes a health check. Verify with:

```bash
docker inspect --format='{{json .State.Health}}' embedcache
```

## Multi-Architecture Build

```bash
# Build for multiple architectures
docker buildx build --platform linux/amd64,linux/arm64 \
  -t embedcache:latest --push .
```
