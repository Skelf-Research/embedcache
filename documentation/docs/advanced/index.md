# Advanced Topics

This section covers advanced usage patterns and customization options.

## Topics

- [Custom Chunkers](custom-chunkers.md) - Implement your own chunking strategies
- [Custom Embedders](custom-embedders.md) - Create custom embedding providers
- [LLM Chunking](llm-chunking.md) - Configure and use LLM-based chunking
- [Performance](performance.md) - Optimization and tuning tips

## When to Use Advanced Features

### Custom Chunkers

Use custom chunkers when:

- Built-in strategies don't fit your use case
- You need domain-specific chunking (e.g., code, legal text)
- You want to integrate with external NLP libraries

### Custom Embedders

Use custom embedders when:

- You need a model not supported by FastEmbed
- You want to use external embedding APIs
- You need custom preprocessing or postprocessing

### LLM Chunking

Use LLM chunking when:

- Semantic coherence is critical
- You have access to LLM infrastructure
- Quality matters more than speed/cost

### Performance Tuning

Focus on performance when:

- Processing large volumes of data
- Running on resource-constrained systems
- Optimizing for production workloads
