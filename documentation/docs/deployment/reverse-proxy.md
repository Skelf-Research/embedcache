# Reverse Proxy Setup

Configure a reverse proxy for production deployments.

## Why Use a Reverse Proxy?

- TLS/HTTPS termination
- Authentication
- Rate limiting
- Load balancing
- Request logging
- CORS handling

## Nginx

### Basic Configuration

Create `/etc/nginx/sites-available/embedcache`:

```nginx
upstream embedcache {
    server 127.0.0.1:8081;
    keepalive 32;
}

server {
    listen 80;
    server_name embedcache.example.com;

    # Redirect to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name embedcache.example.com;

    ssl_certificate /etc/letsencrypt/live/embedcache.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/embedcache.example.com/privkey.pem;

    # SSL settings
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256;
    ssl_prefer_server_ciphers on;

    location / {
        proxy_pass http://embedcache;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header Connection "";

        # Timeouts for embedding generation
        proxy_connect_timeout 60s;
        proxy_send_timeout 300s;
        proxy_read_timeout 300s;
    }
}
```

### With Rate Limiting

```nginx
# Define rate limit zone
limit_req_zone $binary_remote_addr zone=embedcache:10m rate=10r/s;

server {
    # ... SSL configuration ...

    location /v1/embed {
        limit_req zone=embedcache burst=20 nodelay;
        proxy_pass http://embedcache;
        # ... proxy settings ...
    }

    location /v1/process {
        limit_req zone=embedcache burst=5 nodelay;
        proxy_pass http://embedcache;
        # ... proxy settings ...
    }

    location /v1/params {
        proxy_pass http://embedcache;
        # ... proxy settings ...
    }
}
```

### With Basic Authentication

```nginx
server {
    # ... SSL configuration ...

    location / {
        auth_basic "EmbedCache API";
        auth_basic_user_file /etc/nginx/.htpasswd;

        proxy_pass http://embedcache;
        # ... proxy settings ...
    }
}
```

Create password file:

```bash
sudo htpasswd -c /etc/nginx/.htpasswd apiuser
```

### With API Key Authentication

```nginx
server {
    # ... SSL configuration ...

    location / {
        if ($http_x_api_key != "your-secret-api-key") {
            return 401 "Unauthorized";
        }

        proxy_pass http://embedcache;
        # ... proxy settings ...
    }
}
```

## Caddy

### Basic Configuration

Create `Caddyfile`:

```caddyfile
embedcache.example.com {
    reverse_proxy localhost:8081 {
        header_up Host {host}
        header_up X-Real-IP {remote}
        header_up X-Forwarded-For {remote}
        header_up X-Forwarded-Proto {scheme}
    }
}
```

### With Rate Limiting

```caddyfile
embedcache.example.com {
    rate_limit {
        zone embedcache {
            key {remote_host}
            events 10
            window 1s
        }
    }

    reverse_proxy localhost:8081
}
```

### With Basic Auth

```caddyfile
embedcache.example.com {
    basicauth /* {
        apiuser $2a$14$...hashed_password...
    }

    reverse_proxy localhost:8081
}
```

## Traefik

### Docker Compose with Traefik

```yaml
version: '3.8'

services:
  traefik:
    image: traefik:v2.10
    command:
      - "--api.insecure=true"
      - "--providers.docker=true"
      - "--entrypoints.web.address=:80"
      - "--entrypoints.websecure.address=:443"
      - "--certificatesresolvers.letsencrypt.acme.httpchallenge=true"
      - "--certificatesresolvers.letsencrypt.acme.email=admin@example.com"
      - "--certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json"
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - letsencrypt:/letsencrypt

  embedcache:
    build: .
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.embedcache.rule=Host(`embedcache.example.com`)"
      - "traefik.http.routers.embedcache.entrypoints=websecure"
      - "traefik.http.routers.embedcache.tls.certresolver=letsencrypt"
      - "traefik.http.services.embedcache.loadbalancer.server.port=8081"
    volumes:
      - embedcache-data:/data
    environment:
      - SERVER_HOST=0.0.0.0

volumes:
  letsencrypt:
  embedcache-data:
```

## CORS Configuration

If clients access the API from browsers, configure CORS in nginx:

```nginx
location / {
    # CORS headers
    add_header 'Access-Control-Allow-Origin' '*' always;
    add_header 'Access-Control-Allow-Methods' 'GET, POST, OPTIONS' always;
    add_header 'Access-Control-Allow-Headers' 'Content-Type, Authorization, X-API-Key' always;

    # Handle preflight
    if ($request_method = 'OPTIONS') {
        add_header 'Access-Control-Allow-Origin' '*';
        add_header 'Access-Control-Allow-Methods' 'GET, POST, OPTIONS';
        add_header 'Access-Control-Allow-Headers' 'Content-Type, Authorization, X-API-Key';
        add_header 'Access-Control-Max-Age' 86400;
        add_header 'Content-Length' 0;
        return 204;
    }

    proxy_pass http://embedcache;
}
```

## Health Check Endpoint

Configure your proxy to use the health check:

```nginx
upstream embedcache {
    server 127.0.0.1:8081;
    keepalive 32;
}

server {
    location /health {
        proxy_pass http://embedcache/v1/params;
        proxy_connect_timeout 5s;
        proxy_read_timeout 5s;
    }
}
```
