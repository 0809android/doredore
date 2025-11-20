#!/usr/bin/env node
/**
 * doredore + OpenAI Integration Example
 *
 * This example shows how to use doredore with OpenAI's GPT models
 * to build a question-answering system with retrieval-augmented generation.
 *
 * Requirements:
 *   npm install openai
 *   export OPENAI_API_KEY="your-api-key-here"
 */

const { Doredore } = require('doredore');

// Check if OpenAI is available
let OpenAI;
try {
  OpenAI = require('openai').default;
} catch (e) {
  console.error('Error: openai package not found.');
  console.error('Please install it with: npm install openai');
  process.exit(1);
}

// Check for API key
if (!process.env.OPENAI_API_KEY) {
  console.error('Error: OPENAI_API_KEY environment variable not set.');
  console.error('Please set it with: export OPENAI_API_KEY="your-api-key"');
  process.exit(1);
}

console.log('='.repeat(60));
console.log('doredore + OpenAI Integration Example');
console.log('='.repeat(60));
console.log();

// ============================================================================
// 1. Initialize Doredore and OpenAI
// ============================================================================

console.log('1. Initializing...');
const rag = new Doredore('./knowledge.db', 'bge-small-en-v1.5');
const openai = new OpenAI({ apiKey: process.env.OPENAI_API_KEY });
console.log('   ✅ Doredore and OpenAI initialized!');
console.log();

// ============================================================================
// 2. Prepare Knowledge Base
// ============================================================================

console.log('2. Preparing knowledge base...');

try {
  rag.createCollection('faq', 'よくある質問');
} catch (e) {
  // Collection may already exist
}

const documents = [
  '永代供養とは、お墓の管理を寺院に委託する供養形態です。',
  '永代供養の費用は、一般的に10万円〜150万円程度です。',
  '納骨堂には、ロッカー式、仏壇式、自動搬送式などのタイプがあります。',
  '樹木葬は、墓石の代わりに樹木を墓標とする自然葬の一種です。',
  '一般墓は家族代々で受け継がれますが、永代供養墓は寺院が永続的に管理します。'
];

for (const doc of documents) {
  try {
    rag.addDocument(doc, 'faq');
  } catch (e) {
    // Document may already exist
  }
}

console.log('   ✅ Knowledge base ready!');
console.log();

// ============================================================================
// 3. RAG Question Answering Function
// ============================================================================

async function ragAnswer(question) {
  console.log(`Question: "${question}"`);
  console.log();

  // Step 1: Retrieve relevant context using RAG
  console.log('  [RAG] Retrieving relevant documents...');
  const enrichResult = rag.enrich(question, 'faq', null, 3, 0.3);

  console.log(`  [RAG] Found ${enrichResult.sources.length} relevant documents`);
  for (const [index, source] of enrichResult.sources.entries()) {
    console.log(`    ${index + 1}. Score: ${source.score.toFixed(3)}`);
  }
  console.log();

  // Step 2: Call OpenAI with retrieved context
  console.log('  [LLM] Generating answer with GPT-4...');

  const systemPrompt = `あなたは永代供養に関する質問に答えるアシスタントです。
以下の参考情報を基に、正確かつ簡潔に回答してください。

参考情報:
${enrichResult.context}`;

  const response = await openai.chat.completions.create({
    model: 'gpt-4o-mini',  // or gpt-4
    messages: [
      { role: 'system', content: systemPrompt },
      { role: 'user', content: question }
    ],
    temperature: 0.7,
    max_tokens: 500
  });

  const answer = response.choices[0].message.content;

  console.log();
  console.log('Answer:');
  console.log('─'.repeat(60));
  console.log(answer);
  console.log('─'.repeat(60));
  console.log();

  return {
    question,
    answer,
    sources: enrichResult.sources,
    tokensUsed: response.usage.total_tokens
  };
}

// ============================================================================
// 4. Run Examples
// ============================================================================

async function main() {
  console.log('3. Running RAG + OpenAI examples...');
  console.log();
  console.log('='.repeat(60));
  console.log();

  // Example 1
  const result1 = await ragAnswer('永代供養とは何ですか？');
  console.log(`Tokens used: ${result1.tokensUsed}`);
  console.log();
  console.log('='.repeat(60));
  console.log();

  // Example 2
  const result2 = await ragAnswer('永代供養の費用はどれくらいですか？');
  console.log(`Tokens used: ${result2.tokensUsed}`);
  console.log();
  console.log('='.repeat(60));
  console.log();

  // Example 3
  const result3 = await ragAnswer('一般墓との違いを教えてください。');
  console.log(`Tokens used: ${result3.tokensUsed}`);
  console.log();

  console.log('='.repeat(60));
  console.log('✅ All examples completed successfully!');
  console.log('='.repeat(60));
}

main().catch(error => {
  console.error('Error:', error.message);
  process.exit(1);
});
