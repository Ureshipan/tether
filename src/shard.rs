pub type ShardId = String;   // пока String, позже uuid

pub struct Shard {
    pub id: ShardId,
    pub title: String,
    pub body: String,
}

impl Shard {
    pub fn new(id: ShardId, title: String) -> Self {
        Shard { id, title, body: String::new() }
    }
    pub fn summary(&self) -> String {
        format!("[{}] {}", self.id, self.title)
    }
}