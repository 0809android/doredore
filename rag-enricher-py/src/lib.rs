use pyo3::prelude::*;
use pyo3::types::PyDict;
use rag_enricher_core::{Collection, EnrichResult, RAGEnricher as CoreRAGEnricher, SearchResult, SearchMode};
use rag_enricher_core::core::collection::Document;

#[pyclass]
struct PyRAGEnricher {
    inner: CoreRAGEnricher,
}

#[pymethods]
impl PyRAGEnricher {
    #[new]
    #[pyo3(signature = (db_path, model=None, cache_dir=None))]
    fn new(
        db_path: String,
        model: Option<String>,
        cache_dir: Option<String>,
    ) -> PyResult<Self> {
        let inner = CoreRAGEnricher::new(
            db_path,
            model.as_deref(),
            cache_dir.as_deref(),
        )
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(Self { inner })
    }

    // Collection methods

    #[pyo3(signature = (name, description=None))]
    fn create_collection(&self, name: String, description: Option<String>) -> PyResult<i64> {
        self.inner
            .create_collection(&name, description.as_deref())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn get_collection(&self, name: String) -> PyResult<PyCollection> {
        let coll = self
            .inner
            .get_collection(&name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(PyCollection::from(coll))
    }

    fn list_collections(&self) -> PyResult<Vec<PyCollection>> {
        let colls = self
            .inner
            .list_collections()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(colls.into_iter().map(PyCollection::from).collect())
    }

    fn delete_collection(&self, name: String) -> PyResult<bool> {
        self.inner
            .delete_collection(&name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    // Document methods

    #[pyo3(signature = (content, collection="default".to_string(), metadata=None))]
    fn add_document(
        &self,
        content: String,
        collection: String,
        metadata: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<i64> {
        let meta = metadata
            .map(|d| pythonize::depythonize(d.as_any()))
            .transpose()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        self.inner
            .add_document(&content, &collection, meta.as_ref())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    #[pyo3(signature = (documents, collection="default".to_string(), metadata=None))]
    fn add_documents(
        &self,
        documents: Vec<String>,
        collection: String,
        metadata: Option<Vec<Bound<'_, PyDict>>>,
    ) -> PyResult<Vec<i64>> {
        let meta_list = if let Some(meta_vec) = metadata {
            Some(
                meta_vec
                    .iter()
                    .map(|d| pythonize::depythonize(d.as_any()))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?,
            )
        } else {
            None
        };

        self.inner
            .add_documents(documents, &collection, meta_list)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn get_document(&self, document_id: i64) -> PyResult<PyDocument> {
        let doc = self
            .inner
            .get_document(document_id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(PyDocument::from(doc))
    }

    #[pyo3(signature = (collection=None, limit=100, offset=0))]
    fn list_documents(
        &self,
        collection: Option<String>,
        limit: i64,
        offset: i64,
    ) -> PyResult<Vec<PyDocument>> {
        let docs = self
            .inner
            .list_documents(collection.as_deref(), limit, offset)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(docs.into_iter().map(PyDocument::from).collect())
    }

    #[pyo3(signature = (document_id, content=None, metadata=None))]
    fn update_document(
        &self,
        document_id: i64,
        content: Option<String>,
        metadata: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<bool> {
        let meta = metadata
            .map(|d| pythonize::depythonize(d.as_any()))
            .transpose()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        self.inner
            .update_document(document_id, content.as_deref(), meta.as_ref())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn delete_document(&self, document_id: i64) -> PyResult<bool> {
        self.inner
            .delete_document(document_id)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    // Search methods

    #[pyo3(signature = (query, collection=None, collections=None, top_k=5, threshold=0.0, mode="semantic".to_string(), hybrid_weights=None))]
    fn search(
        &self,
        query: String,
        collection: Option<String>,
        collections: Option<Vec<String>>,
        top_k: usize,
        threshold: f32,
        mode: String,
        hybrid_weights: Option<(f32, f32)>,
    ) -> PyResult<Vec<PySearchResult>> {
        // モード文字列をSearchModeに変換
        let search_mode = match mode.to_lowercase().as_str() {
            "semantic" => SearchMode::Semantic,
            "keyword" => SearchMode::Keyword,
            "hybrid" => SearchMode::Hybrid,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Invalid search mode: '{}'. Use 'semantic', 'keyword', or 'hybrid'", mode)
            )),
        };

        let results = self
            .inner
            .search(
                &query,
                collection.as_deref(),
                collections.as_deref(),
                top_k,
                threshold,
                search_mode,
                hybrid_weights,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(results.into_iter().map(PySearchResult::from).collect())
    }

    #[pyo3(signature = (query, collection=None, collections=None, top_k=3, threshold=0.0, mode="semantic".to_string(), hybrid_weights=None))]
    fn enrich(
        &self,
        query: String,
        collection: Option<String>,
        collections: Option<Vec<String>>,
        top_k: usize,
        threshold: f32,
        mode: String,
        hybrid_weights: Option<(f32, f32)>,
    ) -> PyResult<PyEnrichResult> {
        // モード文字列をSearchModeに変換
        let search_mode = match mode.to_lowercase().as_str() {
            "semantic" => SearchMode::Semantic,
            "keyword" => SearchMode::Keyword,
            "hybrid" => SearchMode::Hybrid,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Invalid search mode: '{}'. Use 'semantic', 'keyword', or 'hybrid'", mode)
            )),
        };

        let result = self
            .inner
            .enrich(
                &query,
                collection.as_deref(),
                collections.as_deref(),
                top_k,
                threshold,
                search_mode,
                hybrid_weights,
            )
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(PyEnrichResult::from(result))
    }

    // Import/Export methods

    #[pyo3(signature = (file_path, collection, content_column="content".to_string(), metadata_columns=None))]
    fn import_csv(
        &self,
        file_path: String,
        collection: String,
        content_column: String,
        metadata_columns: Option<Vec<String>>,
    ) -> PyResult<usize> {
        self.inner
            .import_csv(&file_path, &collection, &content_column, metadata_columns)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    #[pyo3(signature = (file_path, collection=None))]
    fn export_csv(&self, file_path: String, collection: Option<String>) -> PyResult<usize> {
        self.inner
            .export_csv(&file_path, collection.as_deref())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

// Python wrapper types

#[pyclass]
#[derive(Clone)]
struct PyCollection {
    #[pyo3(get)]
    id: i64,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    description: Option<String>,
    #[pyo3(get)]
    document_count: i64,
    #[pyo3(get)]
    created_at: String,
    #[pyo3(get)]
    updated_at: String,
}

impl From<Collection> for PyCollection {
    fn from(c: Collection) -> Self {
        Self {
            id: c.id,
            name: c.name,
            description: c.description,
            document_count: c.document_count,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct PyDocument {
    #[pyo3(get)]
    id: i64,
    #[pyo3(get)]
    collection_id: i64,
    #[pyo3(get)]
    collection_name: String,
    #[pyo3(get)]
    content: String,
    #[pyo3(get)]
    created_at: String,
    #[pyo3(get)]
    updated_at: String,
}

#[pymethods]
impl PyDocument {
    #[getter]
    fn metadata(&self, py: Python) -> PyResult<PyObject> {
        Ok(py.None())
    }
}

impl From<Document> for PyDocument {
    fn from(d: Document) -> Self {
        Self {
            id: d.id,
            collection_id: d.collection_id,
            collection_name: d.collection_name,
            content: d.content,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct PySearchResult {
    #[pyo3(get)]
    document_id: i64,
    #[pyo3(get)]
    content: String,
    #[pyo3(get)]
    score: f32,
    #[pyo3(get)]
    collection_name: String,
}

#[pymethods]
impl PySearchResult {
    #[getter]
    fn metadata(&self, py: Python) -> PyResult<PyObject> {
        Ok(py.None())
    }
}

impl From<SearchResult> for PySearchResult {
    fn from(r: SearchResult) -> Self {
        Self {
            document_id: r.document_id,
            content: r.content,
            score: r.score,
            collection_name: r.collection_name,
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct PyEnrichResult {
    #[pyo3(get)]
    question: String,
    #[pyo3(get)]
    context: String,
    #[pyo3(get)]
    sources: Vec<PySearchResult>,
}

impl From<EnrichResult> for PyEnrichResult {
    fn from(r: EnrichResult) -> Self {
        Self {
            question: r.question,
            context: r.context,
            sources: r.sources.into_iter().map(PySearchResult::from).collect(),
        }
    }
}

#[pymodule]
fn rag_enricher(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRAGEnricher>()?;
    m.add_class::<PyCollection>()?;
    m.add_class::<PyDocument>()?;
    m.add_class::<PySearchResult>()?;
    m.add_class::<PyEnrichResult>()?;
    Ok(())
}
