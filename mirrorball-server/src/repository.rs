use std::{
    collections::HashMap,
    num::{NonZeroU64, NonZeroUsize},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::Context;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::{common::ChunkDigest, config::Config, models::Upload, utils};

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum RepositoryKind {
    InMemory,
}

pub trait UploadsRepository: Send + Sync {
    fn new_upload(
        &self,
        destination: PathBuf,
        size: NonZeroU64,
        chunk_hashes: &[ChunkDigest],
    ) -> anyhow::Result<Upload>;

    fn pending_uploads(&self) -> anyhow::Result<Vec<Upload>>;

    fn upload_by_token(&self, token: &str) -> anyhow::Result<Option<Upload>>;
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
        size: NonZeroU64,
        chunk_hashes: &[ChunkDigest],
    ) -> anyhow::Result<Upload> {
        let mut state = self.state.lock().unwrap();

        let cur_id = state.id;

        let token = derive_token(size, chunk_hashes);

        let upload = Upload::new(cur_id, token, destination, size, chunk_hashes.to_vec());
        state.uploads.insert(cur_id, upload.clone());

        state.id = state.id.checked_add(1).context("cannot increment id")?;

        Ok(upload)
    }

    fn pending_uploads(&self) -> anyhow::Result<Vec<Upload>> {
        let state = self.state.lock().unwrap();

        Ok(state
            .uploads
            .values()
            .filter(|upload| upload.is_pending())
            .cloned()
            .collect())
    }

    fn upload_by_token(&self, token: &str) -> anyhow::Result<Option<Upload>> {
        let state = self.state.lock().unwrap();

        Ok(state
            .uploads
            .values()
            .find(|upload| upload.token == token)
            .cloned())
    }
}

pub fn for_config(config: &Config) -> Arc<dyn UploadsRepository> {
    let repo = match config.repo {
        RepositoryKind::InMemory => InMemoryRepository::default(),
    };

    Arc::new(repo)
}

fn derive_token(file_size: NonZeroU64, chunk_hashes: &[ChunkDigest]) -> String {
    let mut token_hasher = Sha256::new();

    let size_bytes = file_size.get().to_be_bytes();
    token_hasher.update(size_bytes);

    for digest in chunk_hashes {
        token_hasher.update(digest.bytes());
    }

    utils::digest::byte_slice_to_hex_string(&token_hasher.finalize())
}
