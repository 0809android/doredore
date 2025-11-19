# Multi-stage build for RAG Enricher Server
FROM rust:1.91-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml ./
COPY rag-enricher-core ./rag-enricher-core
COPY rag-enricher-server ./rag-enricher-server

# Build the server
WORKDIR /app/rag-enricher-server
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary and static files
COPY --from=builder /app/target/release/rag-server /usr/local/bin/
COPY --from=builder /app/rag-enricher-server/static ./static

# Create data directory for database
RUN mkdir -p /data

# Environment variables
ENV DATABASE_PATH=/data/knowledge.db
ENV EMBEDDING_MODEL=bge-small-en-v1.5
ENV HOST=0.0.0.0
ENV PORT=3000
ENV RUST_LOG=info

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Run the server
CMD ["rag-server"]
