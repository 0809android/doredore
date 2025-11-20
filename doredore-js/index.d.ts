/* tslint:disable */
/* eslint-disable */

/**
 * Collection information
 */
export interface Collection {
  /** Collection ID */
  id: number
  /** Collection name */
  name: string
  /** Collection description */
  description?: string
  /** Creation timestamp */
  createdAt: string
}

/**
 * Document information
 */
export interface Document {
  /** Document ID */
  id: number
  /** Collection ID this document belongs to */
  collectionId: number
  /** Document content */
  content: string
  /** Optional metadata (JSON string) */
  metadata?: string
  /** Creation timestamp */
  createdAt: string
}

/**
 * Search result with similarity score
 */
export interface SearchResult {
  /** Document ID */
  documentId: number
  /** Document content */
  content: string
  /** Similarity score (0.0 - 1.0) */
  score: number
  /** Collection name */
  collection: string
  /** Optional metadata (JSON string) */
  metadata?: string
}

/**
 * Enriched result with context for LLM
 */
export interface EnrichResult {
  /** Original query */
  query: string
  /** Generated context for LLM */
  context: string
  /** Source documents used */
  sources: Array<SearchResult>
}

/**
 * doredore - Main class for RAG operations
 */
export class Doredore {
  /**
   * Create a new Doredore instance
   *
   * @param dbPath - Path to SQLite database file
   * @param model - Embedding model name (optional, default: "bge-small-en-v1.5")
   *                Available models:
   *                - "bge-small-en-v1.5" (384 dim, fast)
   *                - "bge-base-en-v1.5" (768 dim, balanced)
   *                - "bge-large-en-v1.5" (1024 dim, accurate)
   *                - "multilingual-e5-small" (384 dim, multilingual)
   *                - "multilingual-e5-base" (768 dim, multilingual)
   * @param cacheDir - Model cache directory (optional)
   */
  constructor(dbPath: string, model?: string, cacheDir?: string)

  // ==========================================================================
  // Collection Management
  // ==========================================================================

  /**
   * Create a new collection
   *
   * @param name - Collection name (must be unique)
   * @param description - Optional description
   * @returns Collection ID
   */
  createCollection(name: string, description?: string): number

  /**
   * Get a collection by name
   *
   * @param name - Collection name
   * @returns Collection information or null if not found
   */
  getCollection(name: string): Collection | null

  /**
   * List all collections
   *
   * @returns Array of all collections
   */
  listCollections(): Array<Collection>

  /**
   * Delete a collection and all its documents
   *
   * @param name - Collection name to delete
   */
  deleteCollection(name: string): void

  // ==========================================================================
  // Document Management
  // ==========================================================================

  /**
   * Add a document to a collection
   *
   * @param content - Document content
   * @param collection - Collection name (optional, default: "default")
   * @param metadata - Optional metadata object
   * @returns Document ID
   *
   * @example
   * ```typescript
   * const docId = rag.addDocument(
   *   "永代供養とは、お墓の管理を寺院に委託する供養形態です。",
   *   "faq",
   *   { category: "永代供養", priority: "high" }
   * );
   * ```
   */
  addDocument(
    content: string,
    collection?: string,
    metadata?: Record<string, any>
  ): number

  /**
   * Get a document by ID
   *
   * @param id - Document ID
   * @returns Document information or null if not found
   */
  getDocument(id: number): Document | null

  /**
   * List documents in a collection
   *
   * @param collection - Collection name (optional, null for all)
   * @param limit - Maximum number of documents (default: 100)
   * @param offset - Offset for pagination (default: 0)
   * @returns Array of documents
   */
  listDocuments(
    collection?: string,
    limit?: number,
    offset?: number
  ): Array<Document>

  /**
   * Delete a document by ID
   *
   * @param id - Document ID to delete
   */
  deleteDocument(id: number): void

  // ==========================================================================
  // Search & Enrich (Main RAG Functions)
  // ==========================================================================

  /**
   * Search for similar documents
   *
   * @param query - Search query
   * @param collection - Single collection to search (optional)
   * @param collections - Multiple collections to search (optional)
   * @param topK - Number of results to return (default: 5)
   * @param threshold - Minimum similarity score (0.0 - 1.0, default: 0.0)
   * @returns Array of search results sorted by similarity
   *
   * @example
   * ```typescript
   * const results = rag.search("永代供養について", "faq", null, 3, 0.5);
   * for (const result of results) {
   *   console.log(`Score: ${result.score}, Content: ${result.content}`);
   * }
   * ```
   */
  search(
    query: string,
    collection?: string,
    collections?: Array<string>,
    topK?: number,
    threshold?: number
  ): Array<SearchResult>

  /**
   * Enrich a query with context from similar documents (Main RAG function)
   *
   * This is the primary function for RAG. It searches for similar documents
   * and generates a formatted context that can be passed to an LLM.
   *
   * @param query - User query
   * @param collection - Single collection to search (optional)
   * @param collections - Multiple collections to search (optional)
   * @param topK - Number of source documents (default: 5)
   * @param threshold - Minimum similarity score (default: 0.0)
   * @returns Enriched result with context for LLM
   *
   * @example
   * ```typescript
   * const result = rag.enrich("永代供養について教えて", "faq", null, 3);
   *
   * // Use with OpenAI
   * const response = await openai.chat.completions.create({
   *   model: "gpt-4",
   *   messages: [
   *     { role: "system", content: `参考情報:\n${result.context}` },
   *     { role: "user", content: result.query }
   *   ]
   * });
   * ```
   */
  enrich(
    query: string,
    collection?: string,
    collections?: Array<string>,
    topK?: number,
    threshold?: number
  ): EnrichResult

  // ==========================================================================
  // CSV Operations
  // ==========================================================================

  /**
   * Import documents from a CSV file
   *
   * @param filePath - Path to CSV file
   * @param collection - Collection name (default: "default")
   * @param contentColumn - Column name for document content (default: "content")
   * @param metadataColumns - Column names to include as metadata (default: [])
   * @returns Number of imported documents
   *
   * @example
   * ```typescript
   * const count = rag.importCsv(
   *   "./faq.csv",
   *   "faq",
   *   "answer",
   *   ["category", "priority"]
   * );
   * console.log(`Imported ${count} documents`);
   * ```
   */
  importCsv(
    filePath: string,
    collection?: string,
    contentColumn?: string,
    metadataColumns?: Array<string>
  ): number

  /**
   * Export documents to a CSV file
   *
   * @param filePath - Path to output CSV file
   * @param collection - Collection name (optional, null for all)
   * @returns Number of exported documents
   *
   * @example
   * ```typescript
   * const count = rag.exportCsv("./export.csv", "faq");
   * console.log(`Exported ${count} documents`);
   * ```
   */
  exportCsv(filePath: string, collection?: string): number
}
