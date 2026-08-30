use std::{
    collections::HashMap,
    num::NonZeroUsize,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::Context;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::{config::Config, models::Upload};

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum RepositoryKind {
    InMemory,
}

pub trait UploadsRepository: Send + Sync {
    fn new_upload(
        &self,
        destination: PathBuf,
        size: NonZeroUsize,
        chunk_hashes: &[String],
    ) -> anyhow::Result<Upload>;

    fn pending_uploads(&self) -> anyhow::Result<Vec<Upload>>;
}

#[derive(Debug)]
pub struct InMemoryRepository {
    state: Mutex<InMemoryState>,
}

#[derive(Debug)]
struct InMemoryState {
    id: NonZeroUsize,
    uploads: HashMap<NonZeroUsize, Upload>,
}

impl Default for InMemoryRepository {
    fn default() -> Self {
        let id_start = unsafe { NonZeroUsize::new_unchecked(1) };

        let state = InMemoryState {
            id: id_start,
            uploads: HashMap::new(),
        };

        Self {
            state: Mutex::new(state),
        }
    }
}

impl UploadsRepository for InMemoryRepository {
    fn new_upload(
        &self,
        destination: PathBuf,
        size: NonZeroUsize,
        chunk_hashes: &[String],
    ) -> anyhow::Result<Upload> {
        let mut state = self.state.lock().unwrap();

        let cur_id = state.id;

        let token = derive_token(size, chunk_hashes);

        let upload = Upload::new(cur_id, token, destination, size);
        state.uploads.insert(cur_id, upload.clone());

        state.id = state.id.checked_add(1).context("cannot increment id")?;

        Ok(upload)
    }

    fn pending_uploads(&self) -> anyhow::Result<Vec<Upload>> {
        let state = self.state.lock().unwrap();

        Ok(state
            .uploads
            .values()
            .cloned()
            .filter(|upload| upload.is_pending())
            .collect())
    }
}

pub fn for_config(config: &Config) -> Arc<dyn UploadsRepository> {
    let repo = match config.repo {
        RepositoryKind::InMemory => InMemoryRepository::default(),
    };

    Arc::new(repo)
}

fn derive_token(file_size: NonZeroUsize, chunk_hashes: &[String]) -> String {
    let mut token_hasher = Sha256::new();

    let size_bytes = file_size.get().to_be_bytes();
    token_hasher.update(size_bytes);

    for hash in chunk_hashes {
        token_hasher.update(hash.as_bytes());
    }

    byte_slice_to_hex_string(&token_hasher.finalize())
}

fn byte_slice_to_hex_string(input: &[u8]) -> String {
    use std::fmt::Write;

    // Each byte is represented as 2 hex characters: 0A, 2C, FF, etc.
    // So we need 2 * the number of input bytes as the capacity of the output String.
    let mut output = String::with_capacity(input.len() * 2);

    for byte in input {
        // Writing a byte to a String should never fail
        write!(&mut output, "{byte:02x}").unwrap()
    }

    output
}
