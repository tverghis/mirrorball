use axum::http::StatusCode;
use mirrorball_api::{CHUNK_HASHES_HEADER, UPLOAD_TOKEN_HEADER};

use crate::api::ApiError;

#[derive(Debug)]
pub enum UploadApiError {
    CreateUpload,
    CreateStagingFile,
    ChunkHashCount(usize),
    InvalidDigest,
    MissingToken,
    MissingChunkHashes,
    UploadNotFound,
    InvalidChunkPlan,
    ChunkHashMismatch,
    InvalidChunkBody,
    WriteChunk,
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
            UploadApiError::CreateStagingFile => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "create_staging_file_failed",
                "Failed to create the upload's staging file",
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
            UploadApiError::MissingToken => ApiError::new(
                StatusCode::BAD_REQUEST,
                "missing_upload_token",
                format!("Missing the {UPLOAD_TOKEN_HEADER} header"),
            ),
            UploadApiError::MissingChunkHashes => ApiError::new(
                StatusCode::BAD_REQUEST,
                "missing_chunk_hashes",
                format!("Missing or empty {CHUNK_HASHES_HEADER} header"),
            ),
            UploadApiError::UploadNotFound => ApiError::new(
                StatusCode::NOT_FOUND,
                "upload_not_found",
                "No upload exists for the supplied token",
            ),
            UploadApiError::InvalidChunkPlan => ApiError::new(
                StatusCode::BAD_REQUEST,
                "invalid_chunk_plan",
                "A supplied chunk hash does not belong to this upload",
            ),
            UploadApiError::ChunkHashMismatch => ApiError::new(
                StatusCode::BAD_REQUEST,
                "chunk_hash_mismatch",
                "A chunk's contents do not match its declared hash",
            ),
            UploadApiError::InvalidChunkBody => ApiError::new(
                StatusCode::BAD_REQUEST,
                "invalid_chunk_body",
                "The request body length does not match the declared chunk hashes",
            ),
            UploadApiError::WriteChunk => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "write_chunk_failed",
                "Failed to write a chunk to the staging file",
            ),
            UploadApiError::Unknown => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "An unknown error occurred",
            ),
        }
    }
}
