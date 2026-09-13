use std::num::{NonZeroU64, NonZeroUsize};

use serde::{Deserialize, Serialize};

pub mod chunks;

pub const UPLOAD_TOKEN_HEADER: &str = "x-mirb-token";
pub const CHUNK_HASHES_HEADER: &str = "x-mirb-chunk-hashes";

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUploadRequest {
    pub file_size: NonZeroU64,
    pub destination: String,
    pub chunk_hashes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUploadResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadChunksResponse {
    pub chunk_count: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PendingUploadsResponse {
    pub item_count: usize,
    pub items: Vec<UploadSummary>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
pub enum UploadState {
    NotStarted,
    InProgress,
    Complete,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadSummary {
    pub id: NonZeroUsize,
    pub destination: String,
    pub size: NonZeroU64,
    pub state: UploadState,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}
