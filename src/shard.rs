use crate::slugify;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use chrono::{DateTime, Utc};

pub type ShardId = String; // пока String, позже uuid

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub created_at: DateTime<Utc>,
    #[serde(default)] pub cwd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shard {
    pub id: ShardId,             // uuid, стабильный
    pub slug: String,            // человекочитаемый, уникальный
    pub title: String,
    pub body: String,
    #[serde(default)] pub links: HashSet<ShardId>,  // по id
    pub context: Context,
}

impl Shard {
    pub fn new(id: ShardId, title: String) -> Self {
        return Shard {
            id,
            unique_slug(title),
            title,
            body: String::new(),
        }
    }
    pub fn summary(&self) -> String {
        return format!("[{}] {}", self.id, self.title)
    }
}
