use std::{
    num::{NonZeroU64, NonZeroUsize},
    path::PathBuf,
};

use jiff::Timestamp;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum UploadState {
    NotStarted,
    InProgress,
    Complete,
}

#[derive(Debug, Clone, Serialize)]
pub struct Upload {
    pub id: NonZeroUsize,
    pub token: String,
    pub destination: PathBuf,
    pub size: NonZeroU64,
    pub state: UploadState,
    pub created_at: Timestamp,
}

impl Upload {
    pub fn new(id: NonZeroUsize, token: String, destination: PathBuf, size: NonZeroU64) -> Self {
        let now = Timestamp::now();

        Self {
            id,
            token,
            destination,
            size,
            state: UploadState::NotStarted,
            created_at: now,
        }
    }

    pub fn is_pending(&self) -> bool {
        self.state != UploadState::Complete
    }
}
