use std::{num::NonZeroUsize, path::PathBuf, time::Duration};

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
    size: usize,
    num_chunks: usize,
    state: UploadState,
    created_at: Timestamp,
    expires_at: Timestamp,
}

impl Upload {
    pub fn new(
        id: NonZeroUsize,
        token: Uuid,
        destination: PathBuf,
        size: usize,
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
            expires_at: now + Duration::from_mins(30),
        }
    }

    pub fn is_pending(&self) -> bool {
        self.state != UploadState::Complete
    }
}
