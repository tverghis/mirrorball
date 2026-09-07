use axum::http::StatusCode;

use crate::api::ApiError;

#[derive(Debug)]
pub enum UploadApiError {
    CreateUpload,
    ChunkHashCount(usize),
    InvalidDigest,
    Unknown,
}

impl From<UploadApiError> for ApiError {
    fn from(err: UploadApiError) -> ApiError {
        match err {
            UploadApiError::CreateUpload => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "create_upload_failed",
                "Failed to create upload",
            ),
            UploadApiError::ChunkHashCount(count) => ApiError::new(
                StatusCode::BAD_REQUEST,
                "invalid_chunk_hash_count",
                format!("Chunk hashes count must be in the range [1, 256] - got {count}"),
            ),
            UploadApiError::InvalidDigest => ApiError::new(
                StatusCode::BAD_REQUEST,
                "invalid_digest",
                "Received an invalid SHA256 digest",
            ),
            UploadApiError::Unknown => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "An unknown error occurred",
            ),
        }
    }
}
