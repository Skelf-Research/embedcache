# User Guide

This section provides comprehensive guides for using EmbedCache.

## Overview

EmbedCache offers two modes of operation:

1. **Service Mode** - Run as a REST API service
2. **Library Mode** - Integrate directly into your Rust application

## Topics

- [Running as Service](running-service.md) - Deploy and manage the EmbedCache service
- [Library Usage](library-usage.md) - Use EmbedCache in your Rust applications
- [Chunking Strategies](chunking.md) - Understand and choose chunking methods
- [Embedding Models](embedding-models.md) - Available models and their characteristics
- [Caching](caching.md) - How caching works and how to manage it

## Quick Reference

### Default Configuration

| Setting | Default Value |
|---------|---------------|
| Chunking Type | `words` |
| Chunk Size | `512` |
| Embedding Model | `BGESmallENV15` |

### Available Chunking Types

| Type | Description |
|------|-------------|
| `words` | Split by whitespace into fixed-size chunks |
| `llm-concept` | LLM-based semantic concept chunking |
| `llm-introspection` | LLM-based analysis and chunking |
