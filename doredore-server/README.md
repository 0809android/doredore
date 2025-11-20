# doredore Server

REST API server and Admin UI for doredore, built with Axum and Alpine.js.

## Features

- ✅ **REST API**: Complete HTTP API for all RAG operations
- ✅ **Admin UI**: Beautiful web interface built with Alpine.js and Tailwind CSS
- ✅ **CORS Support**: Easy integration with any frontend
- ✅ **Docker Ready**: Dockerfile and docker-compose included
- ✅ **Fast & Lightweight**: Built with Axum for maximum performance
- ✅ **Thread-Safe**: Uses Mutex for safe concurrent access

## Quick Start

### Option 1: Run directly

```bash
# Build the server
cargo build --package doredore-server --release

# Run the server
./target/release/rag-server

# Server will start on http://localhost:3000
```

### Option 2: Docker

```bash
cd doredore-server
docker-compose up -d
```

### Option 3: With environment variables

```bash
# Copy example environment
cp .env.example .env

# Edit .env with your settings
# Then run:
./target/release/rag-server
```

## API Endpoints

### Health Check
```bash
GET /health
```

### Collections
```bash
GET    /api/collections          # List all collections
POST   /api/collections          # Create new collection
DELETE /api/collections/:name    # Delete collection
```

### Documents
```bash
GET    /api/documents            # List documents
POST   /api/documents            # Add new document
DELETE /api/documents/:id        # Delete document
```

### Search & RAG
```bash
GET /api/search?q=query&collection=faq&top_k=5&threshold=0.5
GET /api/enrich?q=query&collection=faq&top_k=3
```

### CSV Operations
```bash
POST /api/import-csv
```

## Admin UI

Access the admin interface at `http://localhost:3000/`

Features:
- ✨ Modern, responsive design
- 📝 Manage collections and documents
- 🔍 Test search and RAG functionality
- 📊 View search results with scores
- 💡 Generate LLM context

## API Examples

### Create a collection
```bash
curl -X POST http://localhost:3000/api/collections \
  -H "Content-Type: application/json" \
  -d '{"name": "faq", "description": "FAQs"}'
```

### Add a document
```bash
curl -X POST http://localhost:3000/api/documents \
  -H "Content-Type: application/json" \
  -d '{
    "content": "永代供養とは...",
    "collection": "faq",
    "metadata": {"category": "永代供養", "priority": "high"}
  }'
```

### Search
```bash
curl "http://localhost:3000/api/search?q=永代供養について&collection=faq&top_k=3"
```

### Enrich (RAG)
```bash
curl "http://localhost:3000/api/enrich?q=永代供養について&collection=faq&top_k=3"
```

## Configuration

Environment variables:

```bash
# Database
DATABASE_PATH=./knowledge.db

# Embedding Model
EMBEDDING_MODEL=bge-small-en-v1.5

# Server
HOST=0.0.0.0
PORT=3000

# Logging
RUST_LOG=info
```

## Docker Deployment

```bash
# Build
docker-compose build

# Run
docker-compose up -d

# View logs
docker-compose logs -f

# Stop
docker-compose down
```

## Architecture

```
┌─────────────┐
│   Browser   │
│  (Admin UI) │
└──────┬──────┘
       │
       ↓
┌─────────────────┐
│  Axum Server    │
│  (REST API)     │
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│  doredore   │
│    (Core)       │
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│     SQLite      │
│   (Database)    │
└─────────────────┘
```

## Performance

- **Request latency**: <10ms (typical)
- **Thread-safe**: Yes (Mutex protected)
- **Concurrent requests**: Supported
- **Binary size**: ~20MB (release build)

## Development

```bash
# Run in debug mode
cargo run --package doredore-server

# Run with logs
RUST_LOG=debug cargo run --package doredore-server

# Format code
cargo fmt

# Lint
cargo clippy --package doredore-server
```

## License

MIT License - see LICENSE file for details.
