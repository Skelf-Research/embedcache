# Module Structure

This document describes the source code organization of EmbedCache.

## Overview

```
src/
├── lib.rs                 # Library entry point
├── main.rs                # Binary entry point
├── config.rs              # Server configuration
├── models/                # Data types
├── chunking/              # Text chunking
│   └── llm/               # LLM-based chunking
├── embedding/             # Embedding generation
├── handlers/              # HTTP handlers
├── cache/                 # Caching layer
└── utils/                 # Utility functions
```

## Module Descriptions

### lib.rs

Library entry point that:
- Declares public modules
- Re-exports commonly used types
- Provides `initialize_chunkers()` function

### main.rs

Binary entry point that:
- Loads configuration from environment
- Initializes database, models, chunkers
- Starts Actix-web HTTP server
- Configures API documentation endpoints

### config.rs

Server configuration:
- `ServerConfig` struct
- Environment variable loading
- LLM configuration fields

### models/

Data types used throughout the application:

- **mod.rs** - Module declarations
- **types.rs** - Core types
  - `Config` - Processing configuration
  - `ProcessedContent` - URL processing result
  - `InputData` - URL input
  - `InputDataText` - Text input
  - `get_default_config()` - Default configuration
- **state.rs** - Application state
  - `AppState` - Shared state container

### chunking/

Text chunking functionality:

- **mod.rs** - `ContentChunker` trait
- **word.rs** - `WordChunker` implementation
- **llm/mod.rs** - LLM chunking exports
- **llm/client.rs** - LLM client abstraction
  - `LLMConfig` - Configuration
  - `LLMProvider` - Provider enum
  - `OllamaClient` - Ollama implementation
  - `OpenAIClient` - OpenAI implementation
- **llm/concept.rs** - `LLMConceptChunker`
- **llm/introspection.rs** - `LLMIntrospectionChunker`

### embedding/

Embedding generation:

- **mod.rs** - `Embedder` trait
- **fastembed.rs** - `FastEmbedder` implementation
- **registry.rs** - Model registry
  - `SUPPORTED_MODELS` - Model list
  - `get_embedding_model()` - Model lookup
  - `initialize_models()` - Model initialization

### handlers/

HTTP request handlers:

- **mod.rs** - Module declarations
- **embed.rs** - `embed_text()` handler
- **process.rs** - `process_url()` handler
- **features.rs** - `list_supported_features()` handler

### cache/

Caching layer:

- **mod.rs** - Module declarations
- **sqlite.rs** - SQLite implementation
  - `initialize_db_pool()` - Pool creation
  - `get_from_cache()` - Cache lookup
  - `cache_result()` - Cache storage

### utils/

Utility functions:

- **mod.rs** - Module declarations
- **hash.rs** - `generate_hash()` for cache keys
- **fetch.rs** - `fetch_content()` for URL scraping

## Dependency Graph

```
main.rs
└── lib.rs
    ├── config
    ├── models
    │   └── chunking (for AppState)
    ├── chunking
    │   ├── word
    │   └── llm
    │       ├── client
    │       ├── concept
    │       └── introspection
    ├── embedding
    │   ├── fastembed
    │   └── registry
    ├── handlers
    │   ├── embed
    │   ├── process
    │   └── features
    ├── cache
    │   └── sqlite
    └── utils
        ├── hash
        └── fetch
```

## Adding New Components

### New Chunker

1. Create file in `src/chunking/`
2. Implement `ContentChunker` trait
3. Export in `src/chunking/mod.rs`
4. Register in `initialize_chunkers()` in `lib.rs`

### New Embedder

1. Create file in `src/embedding/`
2. Implement `Embedder` trait
3. Export in `src/embedding/mod.rs`
4. Update handlers to use new embedder

### New Handler

1. Create file in `src/handlers/`
2. Export in `src/handlers/mod.rs`
3. Register route in `main.rs`

### New Utility

1. Create file in `src/utils/`
2. Export in `src/utils/mod.rs`
3. Use in other modules

## Testing

Each module can be tested independently:

```bash
# Test specific module
cargo test chunking::

# Test with logs
RUST_LOG=debug cargo test -- --nocapture
```

## Code Style

- Use `rustfmt` for formatting
- Document public APIs with doc comments
- Use `clippy` for linting
- Keep modules focused and small
