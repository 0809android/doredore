use napi::bindgen_prelude::*;
use napi_derive::napi;
use rag_enricher_core::{
    Collection,
    RAGEnricher as CoreRAGEnricher,
    SearchResult,
    EnrichResult,
    SearchMode,
};
use rag_enricher_core::core::collection::Document;

// ============================================================================
// Collection
// ============================================================================

#[napi(object)]
pub struct JsCollection {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

impl From<Collection> for JsCollection {
    fn from(c: Collection) -> Self {
        Self {
            id: c.id,
            name: c.name,
            description: c.description,
            created_at: c.created_at,
        }
    }
}

// ============================================================================
// Document
// ============================================================================

#[napi(object)]
pub struct JsDocument {
    pub id: i64,
    pub collection_id: i64,
    pub content: String,
    pub metadata: Option<String>,
    pub created_at: String,
}

impl From<Document> for JsDocument {
    fn from(d: Document) -> Self {
        Self {
            id: d.id,
            collection_id: d.collection_id,
            content: d.content,
            metadata: d.metadata.map(|m| m.to_string()),
            created_at: d.created_at,
        }
    }
}

// ============================================================================
// SearchResult
// ============================================================================

#[napi(object)]
pub struct JsSearchResult {
    pub document_id: i64,
    pub content: String,
    pub score: f64,
    pub collection: String,
    pub metadata: Option<String>,
}

impl From<SearchResult> for JsSearchResult {
    fn from(r: SearchResult) -> Self {
        Self {
            document_id: r.document_id,
            content: r.content,
            score: r.score as f64,
            collection: r.collection_name,
            metadata: r.metadata.map(|m| m.to_string()),
        }
    }
}

// ============================================================================
// EnrichResult
// ============================================================================

#[napi(object)]
pub struct JsEnrichResult {
    pub query: String,
    pub context: String,
    pub sources: Vec<JsSearchResult>,
}

impl From<EnrichResult> for JsEnrichResult {
    fn from(r: EnrichResult) -> Self {
        Self {
            query: r.question,
            context: r.context,
            sources: r.sources.into_iter().map(Into::into).collect(),
        }
    }
}

// ============================================================================
// RAGEnricher (Main Class)
// ============================================================================

#[napi]
pub struct RAGEnricher {
    inner: CoreRAGEnricher,
}

#[napi]
impl RAGEnricher {
    /// Create a new RAGEnricher instance
    ///
    /// # Arguments
    /// * `db_path` - Path to SQLite database file
    /// * `model` - Embedding model name (optional, default: "bge-small-en-v1.5")
    /// * `cache_dir` - Model cache directory (optional)
    #[napi(constructor)]
    pub fn new(
        db_path: String,
        model: Option<String>,
        cache_dir: Option<String>,
    ) -> Result<Self> {
        let inner = CoreRAGEnricher::new(
            &db_path,
            model.as_deref(),
            cache_dir.as_deref(),
        )
        .map_err(|e| Error::from_reason(e.to_string()))?;

        Ok(Self { inner })
    }

    // ========================================================================
    // Collection Management
    // ========================================================================

    /// Create a new collection
    #[napi]
    pub fn create_collection(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<i64> {
        self.inner
            .create_collection(&name, description.as_deref())
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Get a collection by name
    #[napi]
    pub fn get_collection(&self, name: String) -> Result<JsCollection> {
        self.inner
            .get_collection(&name)
            .map(Into::into)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// List all collections
    #[napi]
    pub fn list_collections(&self) -> Result<Vec<JsCollection>> {
        self.inner
            .list_collections()
            .map(|collections| collections.into_iter().map(Into::into).collect())
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Delete a collection
    #[napi]
    pub fn delete_collection(&self, name: String) -> Result<bool> {
        self.inner
            .delete_collection(&name)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    // ========================================================================
    // Document Management
    // ========================================================================

    /// Add a document to a collection
    #[napi]
    pub fn add_document(
        &self,
        content: String,
        collection: Option<String>,
        metadata: Option<String>,
    ) -> Result<i64> {
        let collection_name = collection.unwrap_or_else(|| "default".to_string());

        // Parse metadata JSON string to serde_json::Value
        let metadata_value = metadata
            .map(|json_str| {
                serde_json::from_str(&json_str)
                    .map_err(|e| Error::from_reason(format!("Metadata parsing failed: {}", e)))
            })
            .transpose()?;

        self.inner
            .add_document(&content, &collection_name, metadata_value.as_ref())
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Get a document by ID
    #[napi]
    pub fn get_document(&self, id: i64) -> Result<JsDocument> {
        self.inner
            .get_document(id)
            .map(Into::into)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// List documents in a collection
    #[napi]
    pub fn list_documents(
        &self,
        collection: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<JsDocument>> {
        let collection_name = collection.as_deref();
        let limit_val = limit.unwrap_or(100);
        let offset_val = offset.unwrap_or(0);

        self.inner
            .list_documents(collection_name, limit_val, offset_val)
            .map(|docs| docs.into_iter().map(Into::into).collect())
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Delete a document by ID
    #[napi]
    pub fn delete_document(&self, id: i64) -> Result<bool> {
        self.inner
            .delete_document(id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    // ========================================================================
    // Search & Enrich
    // ========================================================================

    /// Search for similar documents
    #[napi]
    pub fn search(
        &self,
        query: String,
        collection: Option<String>,
        collections: Option<Vec<String>>,
        top_k: Option<u32>,
        threshold: Option<f64>,
        mode: Option<String>,
        hybrid_weights: Option<Vec<f64>>,
    ) -> Result<Vec<JsSearchResult>> {
        let top_k_val = top_k.unwrap_or(5) as usize;
        let threshold_val = threshold.unwrap_or(0.0) as f32;
        let mode_str = mode.unwrap_or_else(|| "semantic".to_string());

        // モード文字列をSearchModeに変換
        let search_mode = match mode_str.to_lowercase().as_str() {
            "semantic" => SearchMode::Semantic,
            "keyword" => SearchMode::Keyword,
            "hybrid" => SearchMode::Hybrid,
            _ => return Err(Error::from_reason(
                format!("Invalid search mode: '{}'. Use 'semantic', 'keyword', or 'hybrid'", mode_str)
            )),
        };

        // hybrid_weightsを(f32, f32)に変換
        let weights = hybrid_weights.and_then(|w| {
            if w.len() == 2 {
                Some((w[0] as f32, w[1] as f32))
            } else {
                None
            }
        });

        self.inner
            .search(
                &query,
                collection.as_deref(),
                collections.as_deref(),
                top_k_val,
                threshold_val,
                search_mode,
                weights,
            )
            .map(|results| results.into_iter().map(Into::into).collect())
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Enrich a query with context (main RAG function)
    #[napi]
    pub fn enrich(
        &self,
        query: String,
        collection: Option<String>,
        collections: Option<Vec<String>>,
        top_k: Option<u32>,
        threshold: Option<f64>,
        mode: Option<String>,
        hybrid_weights: Option<Vec<f64>>,
    ) -> Result<JsEnrichResult> {
        let top_k_val = top_k.unwrap_or(5) as usize;
        let threshold_val = threshold.unwrap_or(0.0) as f32;
        let mode_str = mode.unwrap_or_else(|| "semantic".to_string());

        // モード文字列をSearchModeに変換
        let search_mode = match mode_str.to_lowercase().as_str() {
            "semantic" => SearchMode::Semantic,
            "keyword" => SearchMode::Keyword,
            "hybrid" => SearchMode::Hybrid,
            _ => return Err(Error::from_reason(
                format!("Invalid search mode: '{}'. Use 'semantic', 'keyword', or 'hybrid'", mode_str)
            )),
        };

        // hybrid_weightsを(f32, f32)に変換
        let weights = hybrid_weights.and_then(|w| {
            if w.len() == 2 {
                Some((w[0] as f32, w[1] as f32))
            } else {
                None
            }
        });

        self.inner
            .enrich(
                &query,
                collection.as_deref(),
                collections.as_deref(),
                top_k_val,
                threshold_val,
                search_mode,
                weights,
            )
            .map(Into::into)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    // ========================================================================
    // CSV Operations
    // ========================================================================

    /// Import documents from CSV file
    #[napi]
    pub fn import_csv(
        &self,
        file_path: String,
        collection: Option<String>,
        content_column: Option<String>,
        metadata_columns: Option<Vec<String>>,
    ) -> Result<i32> {
        let collection_name = collection.unwrap_or_else(|| "default".to_string());
        let content_col = content_column.unwrap_or_else(|| "content".to_string());
        let metadata_cols = metadata_columns.unwrap_or_else(Vec::new);

        self.inner
            .import_csv(
                &file_path,
                &collection_name,
                &content_col,
                Some(metadata_cols),
            )
            .map(|count| count as i32)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Export documents to CSV file
    #[napi]
    pub fn export_csv(
        &self,
        file_path: String,
        collection: Option<String>,
    ) -> Result<i32> {
        self.inner
            .export_csv(&file_path, collection.as_deref())
            .map(|count| count as i32)
            .map_err(|e| Error::from_reason(e.to_string()))
    }
}
