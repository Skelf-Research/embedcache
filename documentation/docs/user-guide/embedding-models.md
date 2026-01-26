# Embedding Models

EmbedCache supports 22+ embedding models through the FastEmbed library.

## Available Models

### MiniLM Series

Lightweight models suitable for general-purpose use.

| Model | Dimensions | Description |
|-------|------------|-------------|
| `AllMiniLML6V2` | 384 | Fast, general-purpose |
| `AllMiniLML6V2Q` | 384 | Quantized version |
| `AllMiniLML12V2` | 384 | Higher quality |
| `AllMiniLML12V2Q` | 384 | Quantized version |

### BGE Series (BAAI)

High-quality models from BAAI (Beijing Academy of AI).

| Model | Dimensions | Description |
|-------|------------|-------------|
| `BGESmallENV15` | 384 | Small, fast, good quality |
| `BGESmallENV15Q` | 384 | Quantized version |
| `BGEBaseENV15` | 768 | Balanced size/quality |
| `BGEBaseENV15Q` | 768 | Quantized version |
| `BGELargeENV15` | 1024 | Highest quality BGE |
| `BGELargeENV15Q` | 1024 | Quantized version |
| `BGESmallZHV15` | 512 | Chinese language |

### Nomic Series

Models from Nomic AI.

| Model | Dimensions | Description |
|-------|------------|-------------|
| `NomicEmbedTextV1` | 768 | Original Nomic model |
| `NomicEmbedTextV15` | 768 | Improved version |
| `NomicEmbedTextV15Q` | 768 | Quantized version |

### Multilingual E5 Series

Models supporting multiple languages.

| Model | Dimensions | Description |
|-------|------------|-------------|
| `MultilingualE5Small` | 384 | Small multilingual |
| `MultilingualE5Base` | 768 | Base multilingual |
| `MultilingualE5Large` | 1024 | Large multilingual |

### Paraphrase Series

Optimized for paraphrase and similarity detection.

| Model | Dimensions | Description |
|-------|------------|-------------|
| `ParaphraseMLMiniLML12V2` | 384 | MiniLM paraphrase |
| `ParaphraseMLMiniLML12V2Q` | 384 | Quantized version |
| `ParaphraseMLMpnetBaseV2` | 768 | MPNet paraphrase |

### MxbaiEmbed Series

| Model | Dimensions | Description |
|-------|------------|-------------|
| `MxbaiEmbedLargeV1` | 1024 | Large embedding model |
| `MxbaiEmbedLargeV1Q` | 1024 | Quantized version |

## Enabling Models

Configure enabled models in your `.env` file:

```bash
ENABLED_MODELS=BGESmallENV15,AllMiniLML6V2,MultilingualE5Base
```

!!! warning "Memory Usage"
    Each enabled model consumes memory. Only enable models you need.

## Choosing a Model

### By Use Case

| Use Case | Recommended Model |
|----------|-------------------|
| General search | `BGESmallENV15` |
| High accuracy | `BGELargeENV15` |
| Fast processing | `AllMiniLML6V2` |
| Multilingual | `MultilingualE5Base` |
| Low memory | `AllMiniLML6V2Q` |

### By Language

| Language | Recommended Model |
|----------|-------------------|
| English | `BGESmallENV15` |
| Chinese | `BGESmallZHV15` |
| Multiple | `MultilingualE5Base` |

### By Resource Constraints

| Constraint | Recommended Model |
|------------|-------------------|
| Low memory | Quantized models (`*Q`) |
| Fast inference | `AllMiniLML6V2` |
| Best quality | `BGELargeENV15` |

## Model Comparison

```bash
# Benchmark different models
for model in AllMiniLML6V2 BGESmallENV15 BGEBaseENV15; do
  echo "Testing $model..."
  time curl -s -X POST http://localhost:8081/v1/embed \
    -H "Content-Type: application/json" \
    -d "{
      \"text\": [\"Test sentence for benchmarking.\"],
      \"config\": {
        \"chunking_type\": \"words\",
        \"chunking_size\": 512,
        \"embedding_model\": \"$model\"
      }
    }" > /dev/null
done
```

## Quantized Models

Models ending in `Q` are quantized versions that:

- Use less memory
- Are slightly faster
- Have marginally lower quality

Use quantized models when:

- Running on memory-constrained systems
- Processing high volumes
- Quality requirements are flexible
