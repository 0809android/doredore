#!/usr/bin/env node
/**
 * RAG Enricher - Basic Node.js Example
 *
 * This example demonstrates the basic usage of RAG Enricher in Node.js:
 * - Creating a collection
 * - Adding documents
 * - Searching for similar documents
 * - Enriching queries with context
 */

const { RAGEnricher } = require('rag-enricher');

console.log('='.repeat(60));
console.log('RAG Enricher - Basic Node.js Example');
console.log('='.repeat(60));
console.log();

// ============================================================================
// 1. Initialize RAGEnricher
// ============================================================================

console.log('1. Initializing RAGEnricher...');
const rag = new RAGEnricher(
  './knowledge.db',
  'bge-small-en-v1.5',  // Fast, small model (384 dimensions)
  null                   // Use default cache directory
);
console.log('   ✅ Initialization complete!');
console.log();

// ============================================================================
// 2. Create Collection
// ============================================================================

console.log('2. Creating collection...');
try {
  const collectionId = rag.createCollection('faq', 'よくある質問');
  console.log(`   ✅ Collection created! (ID: ${collectionId})`);
} catch (e) {
  console.log(`   ⚠️  Collection may already exist: ${e.message}`);
}
console.log();

// ============================================================================
// 3. Add Documents
// ============================================================================

console.log('3. Adding documents...');

const documents = [
  {
    content: '永代供養とは、お墓の管理を寺院に委託する供養形態です。継承者がいない方でも安心して利用できます。',
    metadata: { category: '永代供養', priority: 'high' }
  },
  {
    content: '永代供養の費用は、一般的に10万円〜150万円程度です。個別安置期間の長さにより価格が変動します。',
    metadata: { category: '料金', priority: 'high' }
  },
  {
    content: '納骨堂には、ロッカー式、仏壇式、自動搬送式などのタイプがあります。都市部で人気が高まっています。',
    metadata: { category: '納骨堂', priority: 'medium' }
  },
  {
    content: '樹木葬は、墓石の代わりに樹木を墓標とする自然葬の一種です。環境に優しく、費用も比較的安価です。',
    metadata: { category: '樹木葬', priority: 'medium' }
  },
  {
    content: '一般墓は家族代々で受け継がれますが、永代供養墓は寺院が永続的に管理します。継承者不要が大きな違いです。',
    metadata: { category: '永代供養', priority: 'medium' }
  }
];

let docCount = 0;
for (const doc of documents) {
  try {
    const docId = rag.addDocument(doc.content, 'faq', doc.metadata);
    docCount++;
    console.log(`   ✅ Document ${docCount} added (ID: ${docId})`);
  } catch (e) {
    console.log(`   ⚠️  Document may already exist: ${e.message}`);
  }
}

console.log();
console.log(`   Total: ${docCount} documents added`);
console.log();

// ============================================================================
// 4. Search for Similar Documents
// ============================================================================

console.log('4. Searching for similar documents...');
console.log('   Query: "永代供養について"');
console.log();

const searchResults = rag.search('永代供養について', 'faq', null, 3, 0.5);

console.log(`   Found ${searchResults.length} results:`);
console.log();

for (const [index, result] of searchResults.entries()) {
  console.log(`   Result ${index + 1}:`);
  console.log(`     Score: ${result.score.toFixed(3)}`);
  console.log(`     Collection: ${result.collection}`);
  console.log(`     Content: ${result.content.substring(0, 50)}...`);
  if (result.metadata) {
    console.log(`     Metadata: ${result.metadata}`);
  }
  console.log();
}

// ============================================================================
// 5. Enrich Query with Context (Main RAG Function)
// ============================================================================

console.log('5. Enriching query with context...');
console.log('   Query: "永代供養について教えて"');
console.log();

const enrichResult = rag.enrich('永代供養について教えて', 'faq', null, 3, 0.0);

console.log('   ✅ Context generated!');
console.log();
console.log('   Generated Context:');
console.log('   ' + '─'.repeat(56));
console.log(enrichResult.context.split('\n').map(line => '   ' + line).join('\n'));
console.log('   ' + '─'.repeat(56));
console.log();

console.log(`   Based on ${enrichResult.sources.length} source document(s):`);
for (const [index, source] of enrichResult.sources.entries()) {
  console.log(`     ${index + 1}. Score: ${source.score.toFixed(3)} - ${source.content.substring(0, 40)}...`);
}
console.log();

// ============================================================================
// 6. Use with LLM (Example)
// ============================================================================

console.log('6. LLM Integration Example:');
console.log('   You can now use this context with any LLM:');
console.log();
console.log('   ```javascript');
console.log('   const response = await openai.chat.completions.create({');
console.log('     model: "gpt-4",');
console.log('     messages: [');
console.log('       {');
console.log('         role: "system",');
console.log('         content: `参考情報:\\n${enrichResult.context}`');
console.log('       },');
console.log('       {');
console.log('         role: "user",');
console.log('         content: enrichResult.query');
console.log('       }');
console.log('     ]');
console.log('   });');
console.log('   ```');
console.log();

// ============================================================================
// 7. Collection Statistics
// ============================================================================

console.log('7. Collection statistics:');
const collections = rag.listCollections();
for (const collection of collections) {
  const docs = rag.listDocuments(collection.name);
  console.log(`   Collection: ${collection.name}`);
  console.log(`     Documents: ${docs.length}`);
  console.log(`     Description: ${collection.description || 'N/A'}`);
  console.log();
}

console.log('='.repeat(60));
console.log('✅ Example completed successfully!');
console.log('='.repeat(60));
console.log();
console.log('Next steps:');
console.log('  - Try the Next.js API route example');
console.log('  - Try the Express integration example');
console.log('  - Integrate with your own LLM (OpenAI, Anthropic, etc.)');
console.log();
