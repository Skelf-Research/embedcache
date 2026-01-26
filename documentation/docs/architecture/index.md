# Architecture

This section describes the architecture and design of EmbedCache.

## Overview

EmbedCache is designed with modularity and extensibility in mind. It can be used as:

1. **Standalone REST API Service** - HTTP endpoints for embedding generation
2. **Rust Library** - Direct integration into Rust applications

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Clients                               │
│              (HTTP Requests / Library Calls)                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      HTTP Layer                              │
│                    (Actix-web Server)                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │ /v1/embed   │  │ /v1/process │  │ /v1/params  │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Core Processing                          │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                    Handlers                          │    │
│  │  ┌─────────┐  ┌─────────────┐  ┌───────────────┐   │    │
│  │  │ Embed   │  │   Process   │  │   Features    │   │    │
│  │  └─────────┘  └─────────────┘  └───────────────┘   │    │
│  └─────────────────────────────────────────────────────┘    │
│                              │                               │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐   │
│  │   Chunking    │  │   Embedding   │  │    Cache      │   │
│  │  ┌─────────┐  │  │  ┌─────────┐  │  │  ┌─────────┐  │   │
│  │  │  Word   │  │  │  │FastEmbed│  │  │  │ SQLite  │  │   │
│  │  │LLM Conc.│  │  │  └─────────┘  │  │  └─────────┘  │   │
│  │  │LLM Intro│  │  │               │  │               │   │
│  │  └─────────┘  │  └───────────────┘  └───────────────┘   │
│  └───────────────┘                                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    External Services                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Ollama    │  │   OpenAI    │  │  Anthropic  │         │
│  │  (LLM API)  │  │  (LLM API)  │  │  (LLM API)  │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
└─────────────────────────────────────────────────────────────┘
```

## Request Flow

### /v1/embed (Text Embedding)

```
1. Receive JSON request with text list
2. Parse configuration (or use defaults)
3. Load appropriate embedding model
4. Generate embeddings via FastEmbed
5. Return embedding vectors
```

### /v1/process (URL Processing)

```
1. Receive JSON request with URL
2. Generate cache key from URL + config
3. Check cache for existing result
4. If cached: return immediately
5. If not cached:
   a. Fetch URL content (readability)
   b. Chunk content (word/LLM chunker)
   c. Generate embeddings
   d. Store in cache
   e. Return result
```

## Topics

- [Design Decisions](design.md) - Why things are built this way
- [Module Structure](modules.md) - Source code organization
