use std::{collections::HashMap, num::NonZeroUsize, path::PathBuf, sync::Mutex};

use anyhow::Context;
use uuid::Uuid;

use crate::models::Upload;

pub trait UploadsRepository: Send + Sync {
    fn new_upload(
        &self,
        token: Uuid,
        destination: PathBuf,
        size: usize,
        num_chunks: usize,
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
        token: Uuid,
        destination: PathBuf,
        size: usize,
        num_chunks: usize,
    ) -> anyhow::Result<Upload> {
        let mut state = self.state.lock().unwrap();

        let cur_id = state.id;

        let upload = Upload::new(cur_id, token, destination, size, num_chunks);
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
