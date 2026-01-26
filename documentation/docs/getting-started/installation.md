# Installation

This guide covers the different ways to install EmbedCache.

## Using Cargo (Recommended)

The easiest way to install EmbedCache is using Cargo:

```bash
cargo install embedcache
```

This installs the `embedcache` binary to your Cargo bin directory.

## Building from Source

To build from source:

```bash
# Clone the repository
git clone https://github.com/skelfresearch/embedcache.git
cd embedcache

# Build in release mode
cargo build --release

# The binary will be at target/release/embedcache
```

## As a Library Dependency

To use EmbedCache as a library in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
embedcache = "0.1.0"
```

## Verifying Installation

After installation, verify it works:

```bash
# Check version
embedcache --version

# Start the server (will download models on first run)
embedcache
```

!!! note "First Run"
    The first time you run EmbedCache, it will download the embedding models. This may take a few minutes depending on your internet connection.

## System Requirements

| Component | Requirement |
|-----------|-------------|
| OS | Linux, macOS, Windows |
| RAM | 2GB minimum, 4GB recommended |
| Disk | 500MB for models (varies by model) |
| Rust | 1.70 or later |

## Optional: LLM Chunking

To use LLM-based chunking strategies, you'll need access to an LLM provider:

- **Ollama** - Local LLM server (free, recommended for development)
- **OpenAI** - OpenAI API access
- **Anthropic** - Anthropic API access

See [LLM Chunking](../advanced/llm-chunking.md) for setup instructions.
