mod net;
mod shard;
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use shard::Shard;

fn main() -> Result<()> {
    let test_shard: Shard = Shard::new("prikol".into(), "Prikol".into());
    let summary: String = test_shard.summary();
    println!("Summary: {summary}");

    Ok(())
}
