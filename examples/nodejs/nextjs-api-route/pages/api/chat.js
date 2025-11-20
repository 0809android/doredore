/**
 * Next.js API Route Example - RAG + OpenAI Chat Endpoint
 *
 * This API route combines doredore with OpenAI to provide
 * a complete question-answering endpoint with retrieval-augmented generation.
 *
 * Usage:
 *   POST /api/chat
 *   Body: { "message": "your question", "collection": "faq" }
 *
 * Environment variables required:
 *   OPENAI_API_KEY
 */

import { Doredore } from 'doredore';
import OpenAI from 'openai';
import path from 'path';

// Initialize services (singleton pattern)
let ragInstance = null;
let openaiInstance = null;

function getRAG() {
  if (!ragInstance) {
    const dbPath = path.join(process.cwd(), 'data', 'knowledge.db');
    ragInstance = new Doredore(dbPath, 'bge-small-en-v1.5');
  }
  return ragInstance;
}

function getOpenAI() {
  if (!openaiInstance) {
    if (!process.env.OPENAI_API_KEY) {
      throw new Error('OPENAI_API_KEY environment variable is not set');
    }
    openaiInstance = new OpenAI({ apiKey: process.env.OPENAI_API_KEY });
  }
  return openaiInstance;
}

export default async function handler(req, res) {
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  try {
    const { message, collection = 'faq', topK = 3 } = req.body;

    if (!message) {
      return res.status(400).json({ error: 'Message is required' });
    }

    // Step 1: Retrieve relevant context using RAG
    const rag = getRAG();
    const enrichResult = rag.enrich(message, collection, null, topK, 0.3);

    console.log(`📚 Retrieved ${enrichResult.sources.length} relevant documents`);

    // Step 2: Generate answer using OpenAI with RAG context
    const openai = getOpenAI();

    const systemPrompt = `あなたは質問に正確に答えるアシスタントです。
以下の参考情報を基に回答してください。

参考情報:
${enrichResult.context}

参考情報に基づいて、簡潔かつ正確に回答してください。`;

    const completion = await openai.chat.completions.create({
      model: 'gpt-4o-mini',
      messages: [
        { role: 'system', content: systemPrompt },
        { role: 'user', content: message }
      ],
      temperature: 0.7,
      max_tokens: 500
    });

    const answer = completion.choices[0].message.content;

    // Return the complete response
    return res.status(200).json({
      success: true,
      message,
      answer,
      sources: enrichResult.sources.map(s => ({
        content: s.content,
        score: s.score,
        metadata: s.metadata ? JSON.parse(s.metadata) : null
      })),
      tokensUsed: completion.usage.total_tokens
    });

  } catch (error) {
    console.error('Chat error:', error);
    return res.status(500).json({
      success: false,
      error: error.message
    });
  }
}
