# Systemd Deployment

Deploy EmbedCache as a systemd service on Linux.

## Prerequisites

1. Install EmbedCache:

```bash
cargo install embedcache
```

2. Create directories:

```bash
sudo mkdir -p /var/lib/embedcache
sudo mkdir -p /etc/embedcache
```

## Configuration File

Create `/etc/embedcache/.env`:

```bash
SERVER_HOST=127.0.0.1
SERVER_PORT=8081
DB_PATH=/var/lib/embedcache/cache.db
DB_JOURNAL_MODE=wal
ENABLED_MODELS=BGESmallENV15,AllMiniLML6V2
```

## Service Unit File

Create `/etc/systemd/system/embedcache.service`:

```ini
[Unit]
Description=EmbedCache - Text Embedding Service
Documentation=https://docs.skelfresearch.com/embedcache/
After=network.target

[Service]
Type=simple
User=embedcache
Group=embedcache
WorkingDirectory=/var/lib/embedcache
EnvironmentFile=/etc/embedcache/.env
ExecStart=/usr/local/cargo/bin/embedcache
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal

# Security hardening
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/var/lib/embedcache
PrivateTmp=yes
PrivateDevices=yes

# Resource limits
MemoryMax=4G
CPUQuota=200%

[Install]
WantedBy=multi-user.target
```

## Setup

### Create Service User

```bash
sudo useradd -r -s /bin/false embedcache
sudo chown -R embedcache:embedcache /var/lib/embedcache
sudo chown -R embedcache:embedcache /etc/embedcache
```

### Enable and Start

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable on boot
sudo systemctl enable embedcache

# Start service
sudo systemctl start embedcache

# Check status
sudo systemctl status embedcache
```

## Management Commands

```bash
# Start
sudo systemctl start embedcache

# Stop
sudo systemctl stop embedcache

# Restart
sudo systemctl restart embedcache

# View logs
sudo journalctl -u embedcache -f

# View recent logs
sudo journalctl -u embedcache --since "1 hour ago"
```

## Log Rotation

Logs are managed by journald. Configure retention in `/etc/systemd/journald.conf`:

```ini
[Journal]
MaxRetentionSec=7d
MaxFileSec=1d
```

## Monitoring

### Health Check Script

Create `/usr/local/bin/embedcache-healthcheck`:

```bash
#!/bin/bash
curl -sf http://localhost:8081/v1/params > /dev/null
exit $?
```

```bash
chmod +x /usr/local/bin/embedcache-healthcheck
```

### With Monitoring Timer

Create `/etc/systemd/system/embedcache-healthcheck.timer`:

```ini
[Unit]
Description=EmbedCache Health Check Timer

[Timer]
OnBootSec=1min
OnUnitActiveSec=1min

[Install]
WantedBy=timers.target
```

Create `/etc/systemd/system/embedcache-healthcheck.service`:

```ini
[Unit]
Description=EmbedCache Health Check

[Service]
Type=oneshot
ExecStart=/usr/local/bin/embedcache-healthcheck
```

## Backup

Create a backup script `/usr/local/bin/embedcache-backup`:

```bash
#!/bin/bash
BACKUP_DIR=/var/backups/embedcache
DATE=$(date +%Y%m%d_%H%M%S)

mkdir -p $BACKUP_DIR
sqlite3 /var/lib/embedcache/cache.db ".backup '$BACKUP_DIR/cache_$DATE.db'"

# Keep only last 7 backups
ls -t $BACKUP_DIR/cache_*.db | tail -n +8 | xargs -r rm
```

Schedule with cron:

```bash
# /etc/cron.d/embedcache
0 2 * * * root /usr/local/bin/embedcache-backup
```

## Troubleshooting

### Service Won't Start

```bash
# Check logs
sudo journalctl -u embedcache -n 50

# Check configuration
sudo -u embedcache cat /etc/embedcache/.env

# Test manually
sudo -u embedcache /usr/local/cargo/bin/embedcache
```

### Permission Issues

```bash
# Fix ownership
sudo chown -R embedcache:embedcache /var/lib/embedcache
sudo chmod 700 /var/lib/embedcache
```

### Port Already in Use

```bash
# Check what's using the port
sudo lsof -i :8081

# Change port in config
sudo nano /etc/embedcache/.env
```
