pub mod core;
pub mod error;

pub use crate::core::{
    collection::Collection,
    database::Database,
    embedding::EmbeddingModel,
    enricher::Doredore,
    search::{SearchResult, EnrichResult, SearchMode},
};
pub use crate::error::{Error, Result};
