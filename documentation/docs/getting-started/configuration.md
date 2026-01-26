# Configuration

EmbedCache is configured through environment variables. You can set these directly or use a `.env` file.

## Configuration File

Create a `.env` file in the same directory where you run EmbedCache:

```bash
# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8081

# Database Configuration
DB_PATH=cache.db
DB_JOURNAL_MODE=wal

# Embedding Models (comma-separated list)
ENABLED_MODELS=AllMiniLML6V2,BGESmallENV15

# LLM Configuration (optional)
# LLM_PROVIDER=ollama
# LLM_MODEL=llama3
# LLM_BASE_URL=http://localhost:11434
# LLM_API_KEY=
# LLM_TIMEOUT=60
```

## Environment Variables

### Server Settings

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_HOST` | `127.0.0.1` | Server bind address |
| `SERVER_PORT` | `8081` | Server port |

### Database Settings

| Variable | Default | Description |
|----------|---------|-------------|
| `DB_PATH` | `cache.db` | SQLite database file path |
| `DB_JOURNAL_MODE` | `wal` | SQLite journal mode (`wal`, `truncate`, `persist`) |

### Model Settings

| Variable | Default | Description |
|----------|---------|-------------|
| `ENABLED_MODELS` | `AllMiniLML6V2` | Comma-separated list of enabled embedding models |

### LLM Settings (Optional)

These settings enable LLM-based chunking strategies.

| Variable | Default | Description |
|----------|---------|-------------|
| `LLM_PROVIDER` | (none) | LLM provider: `ollama`, `openai`, `anthropic` |
| `LLM_MODEL` | `llama3` | Model name to use |
| `LLM_BASE_URL` | (varies) | API base URL |
| `LLM_API_KEY` | (none) | API key (required for OpenAI/Anthropic) |
| `LLM_TIMEOUT` | `60` | Request timeout in seconds |

## Example Configurations

### Minimal Configuration

```bash
# Just use defaults
ENABLED_MODELS=AllMiniLML6V2
```

### Production Configuration

```bash
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
DB_PATH=/var/lib/embedcache/cache.db
DB_JOURNAL_MODE=wal
ENABLED_MODELS=BGESmallENV15,BGEBaseENV15,MultilingualE5Small
```

### With Ollama LLM

```bash
SERVER_HOST=127.0.0.1
SERVER_PORT=8081
DB_PATH=cache.db
ENABLED_MODELS=BGESmallENV15
LLM_PROVIDER=ollama
LLM_MODEL=llama3
LLM_BASE_URL=http://localhost:11434
```

### With OpenAI

```bash
SERVER_HOST=127.0.0.1
SERVER_PORT=8081
DB_PATH=cache.db
ENABLED_MODELS=BGESmallENV15
LLM_PROVIDER=openai
LLM_MODEL=gpt-4o-mini
LLM_API_KEY=sk-your-api-key-here
```

## SQLite Journal Modes

| Mode | Description | Best For |
|------|-------------|----------|
| `wal` | Write-Ahead Logging | High concurrency (recommended) |
| `truncate` | Truncate journal on commit | Single-process access |
| `persist` | Don't delete journal | Systems with slow file deletion |

## Default Request Configuration

When API requests don't specify a configuration, these defaults are used:

```json
{
  "chunking_type": "words",
  "chunking_size": 512,
  "embedding_model": "BGESmallENV15"
}
```
