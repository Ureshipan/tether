mod shard;
use shard::Shard;

fn main() {
    let test_shard: Shard = Shard::new("prikol".into(), "Prikol".into());
    let summary: String = test_shard.summary();
    println!("Summary: {summary}");
}
