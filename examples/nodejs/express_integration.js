#!/usr/bin/env node
/**
 * Express + doredore Integration Example
 *
 * This example shows how to build a REST API server with Express
 * that provides RAG-powered search and question-answering endpoints.
 *
 * Requirements:
 *   npm install express cors
 *
 * Run:
 *   node express_integration.js
 *
 * Endpoints:
 *   POST /api/search - Search for similar documents
 *   POST /api/enrich - Get enriched context for a query
 *   POST /api/documents - Add a new document
 *   GET /api/documents - List all documents
 *   GET /api/collections - List all collections
 */

const express = require('express');
const cors = require('cors');
const { Doredore } = require('doredore');

// ============================================================================
// Setup
// ============================================================================

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(express.json());

// Initialize doredore
const rag = new Doredore(
  './knowledge.db',
  'bge-small-en-v1.5',
  null
);

console.log('✅ Doredore initialized');

// ============================================================================
// API Endpoints
// ============================================================================

/**
 * POST /api/search
 * Search for similar documents
 *
 * Body:
 *   {
 *     "query": "your search query",
 *     "collection": "faq",  // optional
 *     "topK": 5,            // optional
 *     "threshold": 0.5      // optional
 *   }
 */
app.post('/api/search', (req, res) => {
  try {
    const { query, collection, topK = 5, threshold = 0.0 } = req.body;

    if (!query) {
      return res.status(400).json({ error: 'Query is required' });
    }

    const results = rag.search(
      query,
      collection || null,
      null,
      topK,
      threshold
    );

    res.json({
      success: true,
      query,
      results: results.map(r => ({
        documentId: r.documentId,
        content: r.content,
        score: r.score,
        collection: r.collection,
        metadata: r.metadata ? JSON.parse(r.metadata) : null
      })),
      count: results.length
    });

  } catch (error) {
    console.error('Search error:', error);
    res.status(500).json({ error: error.message });
  }
});

/**
 * POST /api/enrich
 * Get enriched context for a query (main RAG function)
 *
 * Body:
 *   {
 *     "query": "your question",
 *     "collection": "faq",  // optional
 *     "topK": 3             // optional
 *   }
 */
app.post('/api/enrich', (req, res) => {
  try {
    const { query, collection, topK = 5, threshold = 0.0 } = req.body;

    if (!query) {
      return res.status(400).json({ error: 'Query is required' });
    }

    const result = rag.enrich(
      query,
      collection || null,
      null,
      topK,
      threshold
    );

    res.json({
      success: true,
      query: result.query,
      context: result.context,
      sources: result.sources.map(s => ({
        documentId: s.documentId,
        content: s.content,
        score: s.score,
        collection: s.collection,
        metadata: s.metadata ? JSON.parse(s.metadata) : null
      })),
      sourceCount: result.sources.length
    });

  } catch (error) {
    console.error('Enrich error:', error);
    res.status(500).json({ error: error.message });
  }
});

/**
 * POST /api/documents
 * Add a new document
 *
 * Body:
 *   {
 *     "content": "document content",
 *     "collection": "faq",           // optional
 *     "metadata": { "key": "value" } // optional
 *   }
 */
app.post('/api/documents', (req, res) => {
  try {
    const { content, collection = 'default', metadata } = req.body;

    if (!content) {
      return res.status(400).json({ error: 'Content is required' });
    }

    const docId = rag.addDocument(content, collection, metadata || null);

    res.json({
      success: true,
      documentId: docId,
      message: 'Document added successfully'
    });

  } catch (error) {
    console.error('Add document error:', error);
    res.status(500).json({ error: error.message });
  }
});

/**
 * GET /api/documents
 * List documents
 *
 * Query params:
 *   collection: filter by collection name
 *   limit: max number of documents (default: 100)
 *   offset: offset for pagination (default: 0)
 */
app.get('/api/documents', (req, res) => {
  try {
    const { collection, limit = 100, offset = 0 } = req.query;

    const documents = rag.listDocuments(
      collection || null,
      parseInt(limit),
      parseInt(offset)
    );

    res.json({
      success: true,
      documents: documents.map(d => ({
        id: d.id,
        collectionId: d.collectionId,
        content: d.content,
        metadata: d.metadata ? JSON.parse(d.metadata) : null,
        createdAt: d.createdAt
      })),
      count: documents.length
    });

  } catch (error) {
    console.error('List documents error:', error);
    res.status(500).json({ error: error.message });
  }
});

/**
 * DELETE /api/documents/:id
 * Delete a document by ID
 */
app.delete('/api/documents/:id', (req, res) => {
  try {
    const { id } = req.params;
    rag.deleteDocument(parseInt(id));

    res.json({
      success: true,
      message: `Document ${id} deleted successfully`
    });

  } catch (error) {
    console.error('Delete document error:', error);
    res.status(500).json({ error: error.message });
  }
});

/**
 * GET /api/collections
 * List all collections
 */
app.get('/api/collections', (req, res) => {
  try {
    const collections = rag.listCollections();

    res.json({
      success: true,
      collections: collections.map(c => ({
        id: c.id,
        name: c.name,
        description: c.description,
        createdAt: c.createdAt
      })),
      count: collections.length
    });

  } catch (error) {
    console.error('List collections error:', error);
    res.status(500).json({ error: error.message });
  }
});

/**
 * POST /api/collections
 * Create a new collection
 */
app.post('/api/collections', (req, res) => {
  try {
    const { name, description } = req.body;

    if (!name) {
      return res.status(400).json({ error: 'Name is required' });
    }

    const collectionId = rag.createCollection(name, description || null);

    res.json({
      success: true,
      collectionId,
      message: 'Collection created successfully'
    });

  } catch (error) {
    console.error('Create collection error:', error);
    res.status(500).json({ error: error.message });
  }
});

/**
 * Health check endpoint
 */
app.get('/health', (req, res) => {
  res.json({ status: 'ok', service: 'doredore-api' });
});

// ============================================================================
// Start Server
// ============================================================================

app.listen(PORT, () => {
  console.log();
  console.log('='.repeat(60));
  console.log('doredore API Server');
  console.log('='.repeat(60));
  console.log();
  console.log(`🚀 Server running on http://localhost:${PORT}`);
  console.log();
  console.log('Available endpoints:');
  console.log(`  POST   http://localhost:${PORT}/api/search`);
  console.log(`  POST   http://localhost:${PORT}/api/enrich`);
  console.log(`  POST   http://localhost:${PORT}/api/documents`);
  console.log(`  GET    http://localhost:${PORT}/api/documents`);
  console.log(`  DELETE http://localhost:${PORT}/api/documents/:id`);
  console.log(`  GET    http://localhost:${PORT}/api/collections`);
  console.log(`  POST   http://localhost:${PORT}/api/collections`);
  console.log(`  GET    http://localhost:${PORT}/health`);
  console.log();
  console.log('='.repeat(60));
});
