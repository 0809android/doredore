use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub document_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: i64,
    pub collection_id: i64,
    pub collection_name: String,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

impl Collection {
    pub fn new(
        id: i64,
        name: String,
        description: Option<String>,
        document_count: i64,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            name,
            description,
            document_count,
            created_at,
            updated_at,
        }
    }
}

impl Document {
    pub fn new(
        id: i64,
        collection_id: i64,
        collection_name: String,
        content: String,
        metadata: Option<serde_json::Value>,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            collection_id,
            collection_name,
            content,
            metadata,
            created_at,
            updated_at,
        }
    }
}
