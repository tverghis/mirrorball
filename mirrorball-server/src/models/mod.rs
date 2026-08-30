use std::{num::NonZeroUsize, path::PathBuf};

use jiff::Timestamp;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum UploadState {
    NotStarted,
    InProgress,
    Complete,
}

#[derive(Debug, Clone, Serialize)]
pub struct Upload {
    id: NonZeroUsize,
    token: Uuid,
    destination: PathBuf,
    size: NonZeroUsize,
    num_chunks: usize,
    state: UploadState,
    created_at: Timestamp,
}

impl Upload {
    pub fn new(
        id: NonZeroUsize,
        token: Uuid,
        destination: PathBuf,
        size: NonZeroUsize,
        num_chunks: usize,
    ) -> Self {
        let now = Timestamp::now();

        Self {
            id,
            token,
            destination,
            size,
            num_chunks,
            state: UploadState::NotStarted,
            created_at: now,
        }
    }

    pub fn is_pending(&self) -> bool {
        self.state != UploadState::Complete
    }
}
