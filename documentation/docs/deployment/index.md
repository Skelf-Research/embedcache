# Deployment

This section covers deploying EmbedCache in production environments.

## Deployment Options

| Option | Best For |
|--------|----------|
| [Docker](docker.md) | Containerized environments |
| [Systemd](systemd.md) | Linux servers |
| [Reverse Proxy](reverse-proxy.md) | Production with Nginx |

## Quick Start

### Docker

```bash
docker run -p 8081:8081 -v embedcache-data:/data embedcache
```

### Systemd

```bash
sudo systemctl start embedcache
```

## Production Checklist

- [ ] Configure proper storage path for database
- [ ] Set up reverse proxy for TLS
- [ ] Configure authentication if needed
- [ ] Set up monitoring and logging
- [ ] Configure backups for cache database
- [ ] Test health checks
- [ ] Load test the deployment
