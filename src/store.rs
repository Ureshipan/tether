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