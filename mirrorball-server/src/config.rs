use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::Deserialize;

use crate::repository::RepositoryKind;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub repo: RepositoryKind,
    pub root: PathBuf,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(filepath: P) -> anyhow::Result<Self> {
        let mut file = File::open(filepath).context("failed to open config file")?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("failed to read config file")?;

        Ok(toml::from_str(&contents).context("failed to deserialize config file")?)
    }
}
