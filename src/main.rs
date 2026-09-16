mod shard; mod net; mod store;
use clap::{Parser, Subcommand};
use anyhow::Result;
use shard::Shard;

#[derive(Parser)]
#[command(name = "tether", about = "Weave shards across the Void")]
struct Cli { #[command(subcommand)] command: Command }

#[derive(Subcommand)]
enum Command {
    /// Create a new shard
    New { title: String },
    /// List all shards
    List,
    /// Show a shard by id
    Show { id: String },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::New { title } => {
            let id = format!("s{}", store::load_all()?.len() + 1);
            store::save_shard(&Shard::new(id.clone(), title))?;
            println!("tethered new shard: {}", id);
        }
        Command::List => for s in store::load_all()? { println!("{}", s.summary()); },
        Command::Show { id } => match store::load_all()?.into_iter().find(|s| s.id == id) {
            Some(s) => println!("{}\n\n{}", s.summary(), s.body),
            None => println!("void: no shard '{}'", id),
        },
    }
    Ok(())
}
