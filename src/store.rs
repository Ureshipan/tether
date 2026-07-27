use std::{path::PathBuf, fs};
use anyhow::{Result, Context};
use directories::ProjectDirs;
use crate::shard::Shard;

pub fn data_dir() -> Result<PathBuf> {
    let proj = ProjectDirs::from("dev", "samos", "tether").context("Cannot determine home dir")?;
    let dir = proj.data_dir().to_path_buf();
    fs::create_dir_all(&dir).context("Cannot create data dir")?;
    return Ok(dir)
}

pub fn save_shard(shard: &Shard) -> Result<()> {
    let path = data_dir()?.join(format!("{}.json", shard.id));
    fs::write(&path, serde_json::to_string_pretty(shard)?)
        .with_context(|| format!("writing {:?}", path))?;
    Ok(())
}

pub fn load_all() -> Result<Vec<Shard>> {
    let mut shards = Vec::new();
    for entry in fs::read_dir(data_dir()?)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let text = fs::read_to_string(&path)?;
            shards.push(serde_json::from_str(&text)?);
        }
    }
    Ok(shards)
}
