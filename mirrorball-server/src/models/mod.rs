use std::{
    num::{NonZeroU64, NonZeroUsize},
    path::PathBuf,
};

use jiff::Timestamp;
use mirrorball_api::{UploadState, UploadSummary};
use serde::Serialize;

use crate::common::ChunkDigest;

#[derive(Debug, Clone, Serialize)]
pub struct Upload {
    pub id: NonZeroUsize,
    pub token: String,
    pub destination: PathBuf,
    pub size: NonZeroU64,
    pub state: UploadState,
    pub created_at: Timestamp,
    #[serde(skip)]
    pub chunk_hashes: Vec<ChunkDigest>,
}

impl Upload {
    pub fn new(
        id: NonZeroUsize,
        token: String,
        destination: PathBuf,
        size: NonZeroU64,
        chunk_hashes: Vec<ChunkDigest>,
    ) -> Self {
        let now = Timestamp::now();

        Self {
            id,
            token,
            destination,
            size,
            state: UploadState::NotStarted,
            created_at: now,
            chunk_hashes,
        }
    }

    pub fn is_pending(&self) -> bool {
        self.state != UploadState::Complete
    }
}

impl From<&Upload> for UploadSummary {
    fn from(upload: &Upload) -> Self {
        Self {
            id: upload.id,
            destination: upload
                .destination
                .clone()
                .into_os_string()
                .into_string()
                .expect("PathBuf should be valid UTF-8"),
            size: upload.size,
            state: upload.state,
        }
    }
}
