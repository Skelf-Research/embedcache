# Troubleshooting

This section helps you diagnose and resolve common issues.

## Quick Diagnostics

### Check Service Health

```bash
curl http://localhost:8081/v1/params
```

### View Logs

```bash
# If running directly
RUST_LOG=debug embedcache

# If using systemd
sudo journalctl -u embedcache -f

# If using Docker
docker logs -f embedcache
```

## Topics

- [Common Issues](common-issues.md) - Frequently encountered problems
- [FAQ](faq.md) - Frequently asked questions

## Getting Help

If you can't resolve your issue:

1. Check the [GitHub Issues](https://github.com/skelfresearch/embedcache/issues)
2. Search for similar problems
3. Create a new issue with:
   - EmbedCache version
   - Operating system
   - Configuration (redact secrets)
   - Error messages
   - Steps to reproduce
