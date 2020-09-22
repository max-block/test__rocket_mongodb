use chrono::{DateTime, Utc};
use mongodb::bson::{self, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    value: i64,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
}



impl Data {
    pub fn new(name: String, value: i64, tags: Vec<String>) -> Self {
        Self {
            id: None,
            name,
            value,
            tags,
            created_at: Utc::now(),
        }
    }

    pub fn to_doc(&self) -> Document {
        bson::to_document(self).unwrap()
    }
}
