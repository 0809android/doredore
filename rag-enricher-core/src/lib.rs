pub mod core;
pub mod error;

pub use crate::core::{
    collection::Collection,
    database::Database,
    embedding::EmbeddingModel,
    enricher::RAGEnricher,
    search::{SearchResult, EnrichResult, SearchMode},
};
pub use crate::error::{Error, Result};
