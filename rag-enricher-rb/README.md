# RAG Enricher - Ruby Bindings

[![Gem Version](https://badge.fury.io/rb/rag-enricher.svg)](https://badge.fury.io/rb/rag-enricher)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Fast and simple Retrieval-Augmented Generation (RAG) library built with Rust, providing Ruby bindings via FFI.

## Features

- **Fast**: Built with Rust for maximum performance
- **Simple**: Clean Ruby API with minimal dependencies
- **Powerful**: Full-text search with vector embeddings
- **Flexible**: Works with any LLM (OpenAI, Anthropic, etc.)
- **Lightweight**: No heavy dependencies, easy to deploy

## Installation

Add to your Gemfile:

```ruby
gem 'ffi'
gem 'rag-enricher', path: 'path/to/rag-enricher-rb'
```

Or install directly:

```bash
gem install ffi
```

## Quick Start

```ruby
require 'rag_enricher'

# Initialize
rag = RAGEnricher::Client.new('./knowledge.db', model: 'bge-small-en-v1.5')

# Create a collection
rag.create_collection('faq', description: 'Frequently asked questions')

# Add documents
rag.add_document(
  '永代供養とは、お墓の管理を寺院に委託する供養形態です。',
  collection: 'faq',
  metadata: { category: '永代供養', priority: 'high' }
)

# Search for similar documents
results = rag.search('永代供養について', collection: 'faq', top_k: 3)
results.each do |result|
  puts "Score: #{result.score}, Content: #{result.content}"
end

# Enrich query with context (main RAG function)
enrich_result = rag.enrich('永代供養について教えて', collection: 'faq', top_k: 3)
puts enrich_result[:context]  # Use this with your LLM
```

## Rails Integration

See `examples/ruby/rails_controller.rb` for complete examples.

### Setup

1. Add to `config/initializers/rag_enricher.rb`:

```ruby
require 'rag_enricher'

RAG = RAGEnricher::Client.new(
  Rails.root.join('db', 'knowledge.db').to_s,
  model: 'bge-small-en-v1.5'
)
```

2. Use in your controllers:

```ruby
class RagController < ApplicationController
  def search
    query = params.require(:query)
    results = RAG.search(query, collection: 'faq', top_k: 5)
    render json: { results: results.map(&:to_h) }
  end

  def enrich
    query = params.require(:query)
    result = RAG.enrich(query, collection: 'faq', top_k: 3)
    render json: { context: result[:context], sources: result[:sources].map(&:to_h) }
  end
end
```

## API Reference

### `RAGEnricher::Client.new(db_path, model: nil, cache_dir: nil)`

Create a new RAG Enricher instance.

**Parameters:**
- `db_path` (String): Path to SQLite database
- `model` (String, optional): Embedding model name (default: 'bge-small-en-v1.5')
  - `bge-small-en-v1.5` (384 dim, fast)
  - `bge-base-en-v1.5` (768 dim, balanced)
  - `bge-large-en-v1.5` (1024 dim, accurate)
  - `multilingual-e5-small` (384 dim, multilingual)
  - `multilingual-e5-base` (768 dim, multilingual)
- `cache_dir` (String, optional): Model cache directory

### Collection Management

#### `create_collection(name, description: nil)`
Create a new collection.

#### `delete_collection(name)`
Delete a collection and all its documents.

### Document Management

#### `add_document(content, collection: 'default', metadata: nil)`
Add a document to a collection.

**Parameters:**
- `content` (String): Document content
- `collection` (String): Collection name
- `metadata` (Hash, optional): Metadata as Ruby hash

**Returns:** Document ID (Integer)

#### `delete_document(id)`
Delete a document by ID.

### Search & Enrich

#### `search(query, collection: nil, top_k: 5, threshold: 0.0)`
Search for similar documents.

**Parameters:**
- `query` (String): Search query
- `collection` (String, optional): Collection to search
- `top_k` (Integer): Number of results
- `threshold` (Float): Minimum similarity score (0.0-1.0)

**Returns:** Array of `SearchResult` objects

#### `enrich(query, collection: nil, top_k: 5, threshold: 0.0)`
Enrich query with context (main RAG function).

**Parameters:** Same as `search`

**Returns:** Hash with keys:
- `:query` - Original query
- `:context` - Formatted context for LLM
- `:sources` - Array of source documents

### CSV Operations

#### `import_csv(file_path, collection: 'default', content_column: 'content')`
Import documents from CSV file.

#### `export_csv(file_path, collection: nil)`
Export documents to CSV file.

## Examples

See the `examples/ruby/` directory for complete examples:
- `basic.rb` - Basic usage
- `rails_controller.rb` - Rails integration

## Building from Source

```bash
# Build the Rust library
cargo build --package rag-enricher-rb --release

# The shared library will be in target/release/
# - macOS: librag_enricher_rb.dylib
# - Linux: librag_enricher_rb.so
# - Windows: rag_enricher_rb.dll
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
