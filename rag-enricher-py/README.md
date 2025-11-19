# RAG Enricher - Python Bindings

Python bindings for RAG Enricher, a lightweight and fast RAG library.

See the main [README](../README.md) for documentation and usage examples.

## Installation

```bash
pip install rag-enricher
```

## Quick Start

```python
from rag_enricher import PyRAGEnricher as RAGEnricher

rag = RAGEnricher("./knowledge.db")
rag.create_collection("faq")
rag.add_document("Your knowledge here", collection="faq")

result = rag.enrich("Your question", collection="faq", top_k=3)
print(result.context)
```
