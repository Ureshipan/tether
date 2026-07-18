use crate::shard::{Shard, ShardId};
use std::collections::HashMap;

pub struct Net {
    shards: HashMap<ShardId, Shard>,
}

impl Net {
    pub fn new() -> Self {
        Net {
            shards: HashMap::new(),
        }
    }
    pub fn add(&mut self, s: Shard) {
        self.shards.insert(s.id.clone(), s);
    }
    pub fn get(&self, id: &str) -> Option<&Shard> {
        self.shards.get(id)
    }
    pub fn remove(&mut self, id: &str) -> Option<Shard> {
        self.shards.remove(id)
    }
    pub fn count(&self) -> usize {
        self.shards.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shard::Shard;
    #[test]
    fn add_and_get() {
        let mut net = Net::new();
        net.add(Shard::new("a".into(), "Alpha".into()));
        assert_eq!(net.count(), 1);
        assert!(net.get("a").is_some());
        assert!(net.get("zzz").is_none());
    }
}
