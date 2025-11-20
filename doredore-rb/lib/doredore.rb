# frozen_string_literal: true

require 'ffi'
require 'json'

module Doredore
  # FFI bindings to the Rust library
  module Native
    extend FFI::Library

    # Load the native library
    lib_name = "libdoredore_rb.#{FFI::Platform::LIBSUFFIX}"
    ffi_lib File.expand_path("../../target/release/#{lib_name}", __dir__)

    # Search result structure
    class CSearchResult < FFI::Struct
      layout :document_id, :long_long,
             :content, :pointer,
             :score, :double,
             :collection, :pointer,
             :metadata, :pointer
    end

    # Search results array
    class CSearchResults < FFI::Struct
      layout :results, :pointer,
             :count, :int
    end

    # Core functions
    attach_function :doredore_new, [:pointer, :pointer, :pointer], :pointer
    attach_function :doredore_free, [:pointer], :void

    # Collection management
    attach_function :doredore_create_collection, [:pointer, :pointer, :pointer], :long_long
    attach_function :doredore_delete_collection, [:pointer, :pointer], :int

    # Document management
    attach_function :doredore_add_document, [:pointer, :pointer, :pointer, :pointer], :long_long
    attach_function :doredore_delete_document, [:pointer, :long_long], :int

    # Search & Enrich
    attach_function :doredore_search, [:pointer, :pointer, :pointer, :int, :double], :pointer
    attach_function :doredore_enrich, [:pointer, :pointer, :pointer, :int, :double], :pointer

    # CSV operations
    attach_function :doredore_import_csv, [:pointer, :pointer, :pointer, :pointer], :int
    attach_function :doredore_export_csv, [:pointer, :pointer, :pointer], :int

    # Memory management
    attach_function :doredore_free_string, [:pointer], :void
    attach_function :doredore_free_search_results, [:pointer], :void
  end

  # Search result Ruby class
  class SearchResult
    attr_reader :document_id, :content, :score, :collection, :metadata

    def initialize(document_id:, content:, score:, collection:, metadata: nil)
      @document_id = document_id
      @content = content
      @score = score
      @collection = collection
      @metadata = metadata
    end

    def to_h
      {
        document_id: @document_id,
        content: @content,
        score: @score,
        collection: @collection,
        metadata: @metadata
      }
    end
  end

  # Main Doredore class
  class Client
    def initialize(db_path, model: 'bge-small-en-v1.5', cache_dir: nil)
      db_ptr = FFI::MemoryPointer.from_string(db_path)
      model_ptr = model ? FFI::MemoryPointer.from_string(model) : nil
      cache_ptr = cache_dir ? FFI::MemoryPointer.from_string(cache_dir) : nil

      @handle = Native.doredore_new(db_ptr, model_ptr, cache_ptr)

      raise 'Failed to initialize Doredore' if @handle.null?

      ObjectSpace.define_finalizer(self, self.class.finalize(@handle))
    end

    def self.finalize(handle)
      proc { Native.doredore_free(handle) }
    end

    # ==================================================================
    # Collection Management
    # ==================================================================

    def create_collection(name, description: nil)
      name_ptr = FFI::MemoryPointer.from_string(name)
      desc_ptr = description ? FFI::MemoryPointer.from_string(description) : nil

      id = Native.doredore_create_collection(@handle, name_ptr, desc_ptr)
      raise "Failed to create collection: #{name}" if id == -1

      id
    end

    def delete_collection(name)
      name_ptr = FFI::MemoryPointer.from_string(name)
      result = Native.doredore_delete_collection(@handle, name_ptr)
      raise "Failed to delete collection: #{name}" if result == -1

      true
    end

    # ==================================================================
    # Document Management
    # ==================================================================

    def add_document(content, collection: 'default', metadata: nil)
      content_ptr = FFI::MemoryPointer.from_string(content)
      collection_ptr = FFI::MemoryPointer.from_string(collection)
      metadata_ptr = metadata ? FFI::MemoryPointer.from_string(metadata.to_json) : nil

      id = Native.doredore_add_document(@handle, content_ptr, collection_ptr, metadata_ptr)
      raise 'Failed to add document' if id == -1

      id
    end

    def delete_document(id)
      result = Native.doredore_delete_document(@handle, id)
      raise "Failed to delete document: #{id}" if result == -1

      true
    end

    # ==================================================================
    # Search & Enrich
    # ==================================================================

    def search(query, collection: nil, top_k: 5, threshold: 0.0)
      query_ptr = FFI::MemoryPointer.from_string(query)
      collection_ptr = collection ? FFI::MemoryPointer.from_string(collection) : nil

      results_ptr = Native.doredore_search(@handle, query_ptr, collection_ptr, top_k, threshold)
      return [] if results_ptr.null?

      results_struct = Native::CSearchResults.new(results_ptr)
      results_array_ptr = results_struct[:results]
      count = results_struct[:count]

      results = []
      (0...count).each do |i|
        result_ptr = results_array_ptr + (i * Native::CSearchResult.size)
        result_struct = Native::CSearchResult.new(result_ptr)

        content = result_struct[:content].read_string
        collection_name = result_struct[:collection].read_string
        metadata_ptr = result_struct[:metadata]
        metadata = metadata_ptr.null? ? nil : JSON.parse(metadata_ptr.read_string)

        results << SearchResult.new(
          document_id: result_struct[:document_id],
          content: content,
          score: result_struct[:score],
          collection: collection_name,
          metadata: metadata
        )
      end

      Native.doredore_free_search_results(results_ptr)
      results
    end

    def enrich(query, collection: nil, top_k: 5, threshold: 0.0)
      query_ptr = FFI::MemoryPointer.from_string(query)
      collection_ptr = collection ? FFI::MemoryPointer.from_string(collection) : nil

      context_ptr = Native.doredore_enrich(@handle, query_ptr, collection_ptr, top_k, threshold)
      raise 'Failed to enrich query' if context_ptr.null?

      context = context_ptr.read_string
      Native.doredore_free_string(context_ptr)

      {
        query: query,
        context: context,
        sources: search(query, collection: collection, top_k: top_k, threshold: threshold)
      }
    end

    # ==================================================================
    # CSV Operations
    # ==================================================================

    def import_csv(file_path, collection: 'default', content_column: 'content')
      file_ptr = FFI::MemoryPointer.from_string(file_path)
      collection_ptr = FFI::MemoryPointer.from_string(collection)
      content_ptr = FFI::MemoryPointer.from_string(content_column)

      count = Native.doredore_import_csv(@handle, file_ptr, collection_ptr, content_ptr)
      raise "Failed to import CSV: #{file_path}" if count == -1

      count
    end

    def export_csv(file_path, collection: nil)
      file_ptr = FFI::MemoryPointer.from_string(file_path)
      collection_ptr = collection ? FFI::MemoryPointer.from_string(collection) : nil

      count = Native.doredore_export_csv(@handle, file_ptr, collection_ptr)
      raise "Failed to export CSV: #{file_path}" if count == -1

      count
    end
  end
end
