/**
 * Next.js API Route Example - RAG Search Endpoint
 *
 * This API route provides a search endpoint that uses RAG Enricher
 * to search for relevant documents and return enriched context.
 *
 * Usage:
 *   POST /api/search
 *   Body: { "query": "your question", "topK": 3 }
 *
 * File location: pages/api/search.js (Next.js App Router)
 * or: app/api/search/route.js (Next.js 13+ App Router)
 */

import { RAGEnricher } from 'rag-enricher';
import path from 'path';

// Initialize RAGEnricher (singleton pattern)
let ragInstance = null;

function getRAG() {
  if (!ragInstance) {
    const dbPath = path.join(process.cwd(), 'data', 'knowledge.db');
    ragInstance = new RAGEnricher(
      dbPath,
      'bge-small-en-v1.5',
      null
    );
    console.log('✅ RAGEnricher initialized');
  }
  return ragInstance;
}

export default async function handler(req, res) {
  // Only allow POST requests
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  try {
    const { query, collection, topK = 5, threshold = 0.0 } = req.body;

    // Validate input
    if (!query || typeof query !== 'string') {
      return res.status(400).json({ error: 'Query is required and must be a string' });
    }

    const rag = getRAG();

    // Enrich the query with context
    const result = rag.enrich(
      query,
      collection || 'faq',
      null,
      topK,
      threshold
    );

    // Return the enriched result
    return res.status(200).json({
      success: true,
      query: result.query,
      context: result.context,
      sources: result.sources.map(source => ({
        documentId: source.documentId,
        content: source.content,
        score: source.score,
        collection: source.collection,
        metadata: source.metadata ? JSON.parse(source.metadata) : null
      })),
      sourceCount: result.sources.length
    });

  } catch (error) {
    console.error('Search error:', error);
    return res.status(500).json({
      success: false,
      error: error.message
    });
  }
}
