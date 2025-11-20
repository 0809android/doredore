#!/usr/bin/env ruby
# frozen_string_literal: true

=begin
doredore - Basic Ruby Example

This example demonstrates the basic usage of doredore in Ruby:
- Creating a collection
- Adding documents
- Searching for similar documents
- Enriching queries with context

Requirements:
  gem install ffi

Usage:
  ruby basic.rb
=end

require_relative '../../doredore-rb/lib/doredore'

puts '=' * 60
puts 'doredore - Basic Ruby Example'
puts '=' * 60
puts

# ============================================================================
# 1. Initialize Doredore
# ============================================================================

puts '1. Initializing Doredore...'
rag = Doredore::Client.new(
  './knowledge.db',
  model: 'bge-small-en-v1.5',  # Fast, small model (384 dimensions)
  cache_dir: nil                # Use default cache directory
)
puts '   ✅ Initialization complete!'
puts

# ============================================================================
# 2. Create Collection
# ============================================================================

puts '2. Creating collection...'
begin
  collection_id = rag.create_collection('faq', description: 'よくある質問')
  puts "   ✅ Collection created! (ID: #{collection_id})"
rescue StandardError => e
  puts "   ⚠️  Collection may already exist: #{e.message}"
end
puts

# ============================================================================
# 3. Add Documents
# ============================================================================

puts '3. Adding documents...'

documents = [
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
]

doc_count = 0
documents.each do |doc|
  begin
    doc_id = rag.add_document(doc[:content], collection: 'faq', metadata: doc[:metadata])
    doc_count += 1
    puts "   ✅ Document #{doc_count} added (ID: #{doc_id})"
  rescue StandardError => e
    puts "   ⚠️  Document may already exist: #{e.message}"
  end
end

puts
puts "   Total: #{doc_count} documents added"
puts

# ============================================================================
# 4. Search for Similar Documents
# ============================================================================

puts '4. Searching for similar documents...'
puts '   Query: "永代供養について"'
puts

search_results = rag.search('永代供養について', collection: 'faq', top_k: 3, threshold: 0.5)

puts "   Found #{search_results.length} results:"
puts

search_results.each_with_index do |result, index|
  puts "   Result #{index + 1}:"
  puts "     Score: #{result.score.round(3)}"
  puts "     Collection: #{result.collection}"
  puts "     Content: #{result.content[0...50]}..."
  puts "     Metadata: #{result.metadata.inspect}" if result.metadata
  puts
end

# ============================================================================
# 5. Enrich Query with Context (Main RAG Function)
# ============================================================================

puts '5. Enriching query with context...'
puts '   Query: "永代供養について教えて"'
puts

enrich_result = rag.enrich('永代供養について教えて', collection: 'faq', top_k: 3, threshold: 0.0)

puts '   ✅ Context generated!'
puts
puts '   Generated Context:'
puts '   ' + '─' * 56
puts enrich_result[:context].split("\n").map { |line| '   ' + line }.join("\n")
puts '   ' + '─' * 56
puts

puts "   Based on #{enrich_result[:sources].length} source document(s):"
enrich_result[:sources].each_with_index do |source, index|
  puts "     #{index + 1}. Score: #{source.score.round(3)} - #{source.content[0...40]}..."
end
puts

# ============================================================================
# 6. Use with LLM (Example)
# ============================================================================

puts '6. LLM Integration Example:'
puts '   You can now use this context with any LLM (OpenAI, Anthropic, etc.):'
puts
puts '   ```ruby'
puts '   require "openai"'
puts
puts '   client = OpenAI::Client.new(access_token: ENV["OPENAI_API_KEY"])'
puts
puts '   response = client.chat('
puts '     parameters: {'
puts '       model: "gpt-4",'
puts '       messages: ['
puts '         {'
puts '           role: "system",'
puts '           content: "参考情報:\n#{enrich_result[:context]}"'
puts '         },'
puts '         {'
puts '           role: "user",'
puts '           content: enrich_result[:query]'
puts '         }'
puts '       ]'
puts '     }'
puts '   )'
puts '   ```'
puts

puts '=' * 60
puts '✅ Example completed successfully!'
puts '=' * 60
puts
puts 'Next steps:'
puts '  - Try the Rails controller integration example'
puts '  - Integrate with your own LLM (OpenAI, Anthropic, etc.)'
puts '  - Build a Rails application with RAG'
puts
