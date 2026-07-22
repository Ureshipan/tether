use serde::{Serialize, Deserialize};

pub type ShardId = String; // пока String, позже uuid

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shard {
    pub id: ShardId,
    pub title: String,
    pub body: String,
}

impl Shard {
    pub fn new(id: ShardId, title: String) -> Self {
        return Shard {
            id,
            title,
            body: String::new(),
        }
    }
    pub fn summary(&self) -> String {
        return format!("[{}] {}", self.id, self.title)
    }
}
