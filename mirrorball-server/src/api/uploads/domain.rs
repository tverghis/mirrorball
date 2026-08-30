use std::{num::NonZeroU64, path::PathBuf};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::models::Upload;

#[derive(Debug)]
pub enum UploadApiError {
    CreateUpload,
    ChunkHashCount(usize),
    InvalidDigest,
    Unknown,
}

impl IntoResponse for UploadApiError {
    fn into_response(self) -> Response {
        match self {
            Self::CreateUpload => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create upload".to_owned(),
            ),
            Self::ChunkHashCount(count) => (
                StatusCode::BAD_REQUEST,
                format!("Chunk hashes count must be in the range [1, 256] - got {count}"),
            ),
            Self::InvalidDigest => (
                StatusCode::BAD_REQUEST,
                "Received an invalid SHA256 digest".to_owned(),
            ),
            Self::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An unknown error occurred".to_owned(),
            ),
        }
        .into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUploadRequest {
    pub file_size: NonZeroU64,
    pub destination: PathBuf,
    pub chunk_hashes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateUploadResponse {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct PendingUploadsResponse {
    pub item_count: usize,
    pub items: Vec<Upload>,
}
