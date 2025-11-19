# frozen_string_literal: true

=begin
RAG Enricher + Rails Integration Example

This example shows how to integrate RAG Enricher with a Rails application.

File location: app/controllers/rag_controller.rb

Installation:
  1. Add to Gemfile:
       gem 'ffi'
       gem 'rag-enricher', path: 'path/to/rag-enricher-rb'

  2. Add to config/initializers/rag_enricher.rb:
       require 'rag_enricher'
       RAG = RAGEnricher::Client.new(
         Rails.root.join('db', 'knowledge.db').to_s,
         model: 'bge-small-en-v1.5'
       )

  3. Add routes:
       post '/api/search', to: 'rag#search'
       post '/api/enrich', to: 'rag#enrich'
       post '/api/chat', to: 'rag#chat'
       resources :documents, only: [:index, :create, :destroy]
=end

class RagController < ApplicationController
  # Skip CSRF for API endpoints (if using as API)
  skip_before_action :verify_authenticity_token, only: [:search, :enrich, :chat]

  # POST /api/search
  # Search for similar documents
  #
  # Parameters:
  #   query: string (required)
  #   collection: string (optional)
  #   top_k: integer (optional, default: 5)
  #   threshold: float (optional, default: 0.0)
  def search
    query = params.require(:query)
    collection = params[:collection]
    top_k = params[:top_k]&.to_i || 5
    threshold = params[:threshold]&.to_f || 0.0

    results = RAG.search(
      query,
      collection: collection,
      top_k: top_k,
      threshold: threshold
    )

    render json: {
      success: true,
      query: query,
      results: results.map(&:to_h),
      count: results.length
    }
  rescue StandardError => e
    render json: {
      success: false,
      error: e.message
    }, status: :internal_server_error
  end

  # POST /api/enrich
  # Get enriched context for a query
  #
  # Parameters:
  #   query: string (required)
  #   collection: string (optional)
  #   top_k: integer (optional, default: 3)
  def enrich
    query = params.require(:query)
    collection = params[:collection]
    top_k = params[:top_k]&.to_i || 3

    result = RAG.enrich(
      query,
      collection: collection,
      top_k: top_k
    )

    render json: {
      success: true,
      query: result[:query],
      context: result[:context],
      sources: result[:sources].map(&:to_h),
      source_count: result[:sources].length
    }
  rescue StandardError => e
    render json: {
      success: false,
      error: e.message
    }, status: :internal_server_error
  end

  # POST /api/chat
  # RAG + OpenAI integration for question answering
  #
  # Parameters:
  #   message: string (required)
  #   collection: string (optional, default: 'faq')
  #   top_k: integer (optional, default: 3)
  #
  # Requires: gem 'ruby-openai'
  def chat
    require 'openai'

    message = params.require(:message)
    collection = params[:collection] || 'faq'
    top_k = params[:top_k]&.to_i || 3

    # Step 1: Get relevant context using RAG
    enrich_result = RAG.enrich(message, collection: collection, top_k: top_k, threshold: 0.3)

    Rails.logger.info "📚 Retrieved #{enrich_result[:sources].length} relevant documents"

    # Step 2: Call OpenAI with context
    client = OpenAI::Client.new(access_token: ENV['OPENAI_API_KEY'])

    system_prompt = <<~PROMPT
      あなたは質問に正確に答えるアシスタントです。
      以下の参考情報を基に回答してください。

      参考情報:
      #{enrich_result[:context]}

      参考情報に基づいて、簡潔かつ正確に回答してください。
    PROMPT

    response = client.chat(
      parameters: {
        model: 'gpt-4o-mini',
        messages: [
          { role: 'system', content: system_prompt },
          { role: 'user', content: message }
        ],
        temperature: 0.7,
        max_tokens: 500
      }
    )

    answer = response.dig('choices', 0, 'message', 'content')

    render json: {
      success: true,
      message: message,
      answer: answer,
      sources: enrich_result[:sources].map(&:to_h),
      tokens_used: response.dig('usage', 'total_tokens')
    }
  rescue StandardError => e
    render json: {
      success: false,
      error: e.message
    }, status: :internal_server_error
  end
end

# ==============================================================================
# Documents Controller
# ==============================================================================

class DocumentsController < ApplicationController
  skip_before_action :verify_authenticity_token

  # GET /documents
  # List all documents
  #
  # Parameters:
  #   collection: string (optional)
  def index
    # Note: You'll need to implement list_documents in the Ruby binding
    # or use the database directly

    # For now, return a placeholder
    render json: {
      success: true,
      documents: [],
      message: 'List documents - to be implemented'
    }
  end

  # POST /documents
  # Add a new document
  #
  # Parameters:
  #   content: string (required)
  #   collection: string (optional, default: 'default')
  #   metadata: hash (optional)
  def create
    content = params.require(:content)
    collection = params[:collection] || 'default'
    metadata = params[:metadata]

    doc_id = RAG.add_document(
      content,
      collection: collection,
      metadata: metadata
    )

    render json: {
      success: true,
      document_id: doc_id,
      message: 'Document added successfully'
    }
  rescue StandardError => e
    render json: {
      success: false,
      error: e.message
    }, status: :internal_server_error
  end

  # DELETE /documents/:id
  # Delete a document
  def destroy
    id = params.require(:id).to_i
    RAG.delete_document(id)

    render json: {
      success: true,
      message: "Document #{id} deleted successfully"
    }
  rescue StandardError => e
    render json: {
      success: false,
      error: e.message
    }, status: :internal_server_error
  end
end

# ==============================================================================
# Background Job Example (Sidekiq)
# ==============================================================================

class ImportCsvJob
  include Sidekiq::Worker

  def perform(file_path, collection = 'default')
    # Initialize RAG (or use global instance)
    rag = RAGEnricher::Client.new(
      Rails.root.join('db', 'knowledge.db').to_s,
      model: 'bge-small-en-v1.5'
    )

    # Import CSV
    count = rag.import_csv(file_path, collection: collection, content_column: 'content')

    Rails.logger.info "✅ Imported #{count} documents from #{file_path}"
  rescue StandardError => e
    Rails.logger.error "❌ CSV import failed: #{e.message}"
    raise
  end
end
