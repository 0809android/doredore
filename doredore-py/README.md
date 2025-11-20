# doredore - Python Bindings

Python bindings for doredore, a lightweight and fast RAG library.

See the main [README](../README.md) for documentation and usage examples.

## Installation

```bash
pip install doredore
```

## Quick Start

```python
from doredore import PyDoredore as Doredore

rag = Doredore("./knowledge.db")
rag.create_collection("faq")
rag.add_document("Your knowledge here", collection="faq")

result = rag.enrich("Your question", collection="faq", top_k=3)
print(result.context)
```
