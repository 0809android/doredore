use rag_enricher_core::core::enricher::RAGEnricher as CoreRAGEnricher;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_longlong};
use std::ptr;

// ============================================================================
// Type Definitions
// ============================================================================

/// Opaque handle to RAGEnricher instance
pub struct RAGEnricher {
    inner: CoreRAGEnricher,
}

/// Search result structure for C FFI
#[repr(C)]
pub struct CSearchResult {
    pub document_id: c_longlong,
    pub content: *mut c_char,
    pub score: c_double,
    pub collection: *mut c_char,
    pub metadata: *mut c_char,
}

/// Array of search results
#[repr(C)]
pub struct CSearchResults {
    pub results: *mut CSearchResult,
    pub count: c_int,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert Rust String to C string (caller must free)
unsafe fn to_c_string(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

/// Convert C string to Rust String
unsafe fn from_c_string(s: *const c_char) -> String {
    if s.is_null() {
        return String::new();
    }
    CStr::from_ptr(s).to_string_lossy().into_owned()
}

// ============================================================================
// Core Functions
// ============================================================================

/// Create a new RAGEnricher instance
///
/// # Safety
/// Caller must call rag_enricher_free() to deallocate
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_new(
    db_path: *const c_char,
    model: *const c_char,
    cache_dir: *const c_char,
) -> *mut RAGEnricher {
    let db_path_str = from_c_string(db_path);
    let model_str = if model.is_null() {
        None
    } else {
        Some(from_c_string(model))
    };
    let cache_str = if cache_dir.is_null() {
        None
    } else {
        Some(from_c_string(cache_dir))
    };

    match CoreRAGEnricher::new(
        &db_path_str,
        model_str.as_deref(),
        cache_str.as_deref(),
    ) {
        Ok(enricher) => Box::into_raw(Box::new(RAGEnricher { inner: enricher })),
        Err(_) => ptr::null_mut(),
    }
}

/// Free a RAGEnricher instance
///
/// # Safety
/// Must only be called once per instance
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_free(rag: *mut RAGEnricher) {
    if !rag.is_null() {
        drop(Box::from_raw(rag));
    }
}

// ============================================================================
// Collection Management
// ============================================================================

/// Create a new collection
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_create_collection(
    rag: *mut RAGEnricher,
    name: *const c_char,
    description: *const c_char,
) -> c_longlong {
    if rag.is_null() {
        return -1;
    }

    let enricher = &(*rag).inner;
    let name_str = from_c_string(name);
    let desc_str = if description.is_null() {
        None
    } else {
        Some(from_c_string(description))
    };

    match enricher.create_collection(&name_str, desc_str.as_deref()) {
        Ok(id) => id,
        Err(_) => -1,
    }
}

/// Delete a collection
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_delete_collection(
    rag: *mut RAGEnricher,
    name: *const c_char,
) -> c_int {
    if rag.is_null() {
        return -1;
    }

    let enricher = &(*rag).inner;
    let name_str = from_c_string(name);

    match enricher.delete_collection(&name_str) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

// ============================================================================
// Document Management
// ============================================================================

/// Add a document to a collection
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_add_document(
    rag: *mut RAGEnricher,
    content: *const c_char,
    collection: *const c_char,
    metadata: *const c_char,
) -> c_longlong {
    if rag.is_null() {
        return -1;
    }

    let enricher = &(*rag).inner;
    let content_str = from_c_string(content);
    let collection_str = if collection.is_null() {
        "default".to_string()
    } else {
        from_c_string(collection)
    };
    let metadata_json = if metadata.is_null() {
        None
    } else {
        let metadata_str = from_c_string(metadata);
        match serde_json::from_str(&metadata_str) {
            Ok(json) => Some(json),
            Err(_) => return -1,
        }
    };

    match enricher.add_document(&content_str, &collection_str, metadata_json.as_ref()) {
        Ok(id) => id,
        Err(_) => -1,
    }
}

/// Delete a document by ID
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_delete_document(
    rag: *mut RAGEnricher,
    id: c_longlong,
) -> c_int {
    if rag.is_null() {
        return -1;
    }

    let enricher = &(*rag).inner;

    match enricher.delete_document(id) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

// ============================================================================
// Search & Enrich
// ============================================================================

/// Search for similar documents
///
/// # Parameters
/// * mode - Search mode: "semantic", "keyword", or "hybrid" (default: "semantic")
/// * semantic_weight - Weight for semantic score in hybrid mode (default: 0.7)
/// * keyword_weight - Weight for keyword score in hybrid mode (default: 0.3)
///
/// # Safety
/// Caller must call rag_enricher_free_search_results() to deallocate
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_search(
    rag: *mut RAGEnricher,
    query: *const c_char,
    collection: *const c_char,
    top_k: c_int,
    threshold: c_double,
    mode: *const c_char,
    semantic_weight: c_double,
    keyword_weight: c_double,
) -> *mut CSearchResults {
    if rag.is_null() {
        return ptr::null_mut();
    }

    let enricher = &(*rag).inner;
    let query_str = from_c_string(query);
    let collection_str = if collection.is_null() {
        None
    } else {
        Some(from_c_string(collection))
    };

    // モード文字列をSearchModeに変換
    use rag_enricher_core::SearchMode;
    let mode_str = if mode.is_null() {
        "semantic".to_string()
    } else {
        from_c_string(mode)
    };

    let search_mode = match mode_str.to_lowercase().as_str() {
        "semantic" => SearchMode::Semantic,
        "keyword" => SearchMode::Keyword,
        "hybrid" => SearchMode::Hybrid,
        _ => SearchMode::Semantic, // デフォルトにフォールバック
    };

    // hybrid_weightsを設定（デフォルト: 0.7, 0.3）
    let weights = if semantic_weight > 0.0 && keyword_weight > 0.0 {
        Some((semantic_weight as f32, keyword_weight as f32))
    } else {
        None
    };

    let results = match enricher.search(
        &query_str,
        collection_str.as_deref(),
        None,
        top_k as usize,
        threshold as f32,
        search_mode,
        weights,
    ) {
        Ok(r) => r,
        Err(_) => return ptr::null_mut(),
    };

    // Convert results to C format
    let mut c_results: Vec<CSearchResult> = results
        .into_iter()
        .map(|r| CSearchResult {
            document_id: r.document_id,
            content: to_c_string(r.content),
            score: r.score as c_double,
            collection: to_c_string(r.collection_name),
            metadata: if let Some(m) = r.metadata {
                to_c_string(m.to_string())
            } else {
                ptr::null_mut()
            },
        })
        .collect();

    let count = c_results.len() as c_int;
    let results_ptr = c_results.as_mut_ptr();
    std::mem::forget(c_results);

    Box::into_raw(Box::new(CSearchResults {
        results: results_ptr,
        count,
    }))
}

/// Get enriched context for a query (main RAG function)
///
/// # Parameters
/// * mode - Search mode: "semantic", "keyword", or "hybrid" (default: "semantic")
/// * semantic_weight - Weight for semantic score in hybrid mode (default: 0.7)
/// * keyword_weight - Weight for keyword score in hybrid mode (default: 0.3)
///
/// # Safety
/// Caller must call rag_enricher_free_string() on the returned string
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_enrich(
    rag: *mut RAGEnricher,
    query: *const c_char,
    collection: *const c_char,
    top_k: c_int,
    threshold: c_double,
    mode: *const c_char,
    semantic_weight: c_double,
    keyword_weight: c_double,
) -> *mut c_char {
    if rag.is_null() {
        return ptr::null_mut();
    }

    let enricher = &(*rag).inner;
    let query_str = from_c_string(query);
    let collection_str = if collection.is_null() {
        None
    } else {
        Some(from_c_string(collection))
    };

    // モード文字列をSearchModeに変換
    use rag_enricher_core::SearchMode;
    let mode_str = if mode.is_null() {
        "semantic".to_string()
    } else {
        from_c_string(mode)
    };

    let search_mode = match mode_str.to_lowercase().as_str() {
        "semantic" => SearchMode::Semantic,
        "keyword" => SearchMode::Keyword,
        "hybrid" => SearchMode::Hybrid,
        _ => SearchMode::Semantic, // デフォルトにフォールバック
    };

    // hybrid_weightsを設定（デフォルト: 0.7, 0.3）
    let weights = if semantic_weight > 0.0 && keyword_weight > 0.0 {
        Some((semantic_weight as f32, keyword_weight as f32))
    } else {
        None
    };

    match enricher.enrich(
        &query_str,
        collection_str.as_deref(),
        None,
        top_k as usize,
        threshold as f32,
        search_mode,
        weights,
    ) {
        Ok(result) => to_c_string(result.context),
        Err(_) => ptr::null_mut(),
    }
}

// ============================================================================
// CSV Operations
// ============================================================================

/// Import documents from CSV file
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_import_csv(
    rag: *mut RAGEnricher,
    file_path: *const c_char,
    collection: *const c_char,
    content_column: *const c_char,
) -> c_int {
    if rag.is_null() {
        return -1;
    }

    let enricher = &(*rag).inner;
    let file_str = from_c_string(file_path);
    let collection_str = if collection.is_null() {
        "default".to_string()
    } else {
        from_c_string(collection)
    };
    let content_col = if content_column.is_null() {
        "content".to_string()
    } else {
        from_c_string(content_column)
    };

    match enricher.import_csv(&file_str, &collection_str, &content_col, None) {
        Ok(count) => count as c_int,
        Err(_) => -1,
    }
}

/// Export documents to CSV file
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_export_csv(
    rag: *mut RAGEnricher,
    file_path: *const c_char,
    collection: *const c_char,
) -> c_int {
    if rag.is_null() {
        return -1;
    }

    let enricher = &(*rag).inner;
    let file_str = from_c_string(file_path);
    let collection_str = if collection.is_null() {
        None
    } else {
        Some(from_c_string(collection))
    };

    match enricher.export_csv(&file_str, collection_str.as_deref()) {
        Ok(count) => count as c_int,
        Err(_) => -1,
    }
}

// ============================================================================
// Memory Management
// ============================================================================

/// Free a C string allocated by this library
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

/// Free search results
#[no_mangle]
pub unsafe extern "C" fn rag_enricher_free_search_results(results: *mut CSearchResults) {
    if results.is_null() {
        return;
    }

    let results_box = Box::from_raw(results);
    let results_vec =
        Vec::from_raw_parts(results_box.results, results_box.count as usize, results_box.count as usize);

    for result in results_vec {
        rag_enricher_free_string(result.content);
        rag_enricher_free_string(result.collection);
        if !result.metadata.is_null() {
            rag_enricher_free_string(result.metadata);
        }
    }
}
