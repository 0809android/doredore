use crate::error::{Error, Result};
use fastembed::{EmbeddingModel as FastEmbedModel, InitOptions, TextEmbedding};
use std::sync::Arc;

pub struct EmbeddingModel {
    model: Arc<TextEmbedding>,
    dimension: usize,
}

impl EmbeddingModel {
    pub fn new(model_name: Option<&str>, cache_dir: Option<&str>) -> Result<Self> {
        let model_type = match model_name {
            Some("bge-small-en-v1.5") | None => FastEmbedModel::BGESmallENV15,
            Some("bge-base-en-v1.5") => FastEmbedModel::BGEBaseENV15,
            Some("bge-large-en-v1.5") => FastEmbedModel::BGELargeENV15,
            Some("multilingual-e5-small") => FastEmbedModel::MultilingualE5Small,
            Some("multilingual-e5-base") => FastEmbedModel::MultilingualE5Base,
            Some(name) => {
                return Err(Error::InvalidInput(format!(
                    "Unsupported model: {}",
                    name
                )))
            }
        };

        let dimension = Self::get_model_dimension(&model_type);

        let mut options = InitOptions::new(model_type);
        if let Some(dir) = cache_dir {
            options = options.with_cache_dir(dir.into());
        }

        let model = TextEmbedding::try_new(options)
            .map_err(|e| Error::Embedding(format!("Failed to initialize embedding model: {}", e)))?;

        Ok(Self {
            model: Arc::new(model),
            dimension,
        })
    }

    fn get_model_dimension(model: &FastEmbedModel) -> usize {
        match model {
            FastEmbedModel::BGESmallENV15 => 384,
            FastEmbedModel::BGEBaseENV15 => 768,
            FastEmbedModel::BGELargeENV15 => 1024,
            FastEmbedModel::MultilingualE5Small => 384,
            FastEmbedModel::MultilingualE5Base => 768,
            _ => 384, // デフォルト
        }
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self
            .model
            .embed(vec![text.to_string()], None)
            .map_err(|e| Error::Embedding(format!("Failed to generate embedding: {}", e)))?;

        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| Error::Embedding("No embedding generated".to_string()))
    }

    pub fn embed_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let embeddings = self
            .model
            .embed(texts, None)
            .map_err(|e| Error::Embedding(format!("Failed to generate embeddings: {}", e)))?;

        Ok(embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_model_initialization() {
        let model = EmbeddingModel::new(Some("bge-small-en-v1.5"), None);
        assert!(model.is_ok());
    }

    #[test]
    fn test_embed_single_text() {
        let model = EmbeddingModel::new(Some("bge-small-en-v1.5"), None).unwrap();
        let result = model.embed("Hello, world!");
        assert!(result.is_ok());
        let embedding = result.unwrap();
        assert_eq!(embedding.len(), 384);
    }

    #[test]
    fn test_embed_batch() {
        let model = EmbeddingModel::new(Some("bge-small-en-v1.5"), None).unwrap();
        let texts = vec!["Hello".to_string(), "World".to_string()];
        let result = model.embed_batch(texts);
        assert!(result.is_ok());
        let embeddings = result.unwrap();
        assert_eq!(embeddings.len(), 2);
        assert_eq!(embeddings[0].len(), 384);
    }
}
