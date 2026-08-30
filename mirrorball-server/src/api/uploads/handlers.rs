use std::sync::Arc;

use axum::{Json, extract::State};

use super::domain::*;
use crate::repository::UploadsRepository;

pub async fn create_upload_request(
    State(repo): State<Arc<dyn UploadsRepository>>,
    Json(body): Json<CreateUploadRequest>,
) -> Result<Json<CreateUploadResponse>, UploadApiError> {
    let num_chunk_hashes = body.chunk_hashes.len();

    if !(1..=256).contains(&num_chunk_hashes) {
        return Err(UploadApiError::ChunkHashCount(num_chunk_hashes));
    }

    let upload = repo
        .new_upload(body.destination, body.file_size, &body.chunk_hashes)
        .map_err(|_| UploadApiError::CreateUpload)?;

    Ok(Json(CreateUploadResponse {
        token: upload.token,
    }))
}

pub async fn get_pending_uploads(
    State(repo): State<Arc<dyn UploadsRepository>>,
) -> Result<Json<PendingUploadsResponse>, UploadApiError> {
    let pending_uploads = repo
        .pending_uploads()
        .map_err(|_| UploadApiError::Unknown)?;

    Ok(Json(PendingUploadsResponse {
        item_count: pending_uploads.len(),
        items: pending_uploads,
    }))
}
