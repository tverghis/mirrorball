use std::{path::PathBuf, sync::Arc};

use axum::{Json, extract::State};
use mirrorball_api::{
    CreateUploadRequest, CreateUploadResponse, PendingUploadsResponse, UploadSummary,
};

use super::domain::*;
use crate::{common::ChunkDigest, repository::UploadsRepository};

pub async fn create_upload_request(
    State(repo): State<Arc<dyn UploadsRepository>>,
    Json(body): Json<CreateUploadRequest>,
) -> Result<Json<CreateUploadResponse>, UploadApiError> {
    let num_chunk_hashes = body.chunk_hashes.len();

    if !(1..=256).contains(&num_chunk_hashes) {
        return Err(UploadApiError::ChunkHashCount(num_chunk_hashes));
    }

    let digests: Result<Vec<_>, anyhow::Error> = body
        .chunk_hashes
        .iter()
        .map(|s| ChunkDigest::try_from(s.as_str()))
        .collect();

    let digests = digests.map_err(|_| UploadApiError::InvalidDigest)?;

    let destination_path = PathBuf::from(body.destination);

    let upload = repo
        .new_upload(destination_path, body.file_size, &digests)
        .map_err(|_| UploadApiError::CreateUpload)?;

    Ok(Json(CreateUploadResponse {
        token: upload.token,
    }))
}

pub async fn get_pending_uploads(
    State(repo): State<Arc<dyn UploadsRepository>>,
) -> Result<Json<PendingUploadsResponse>, UploadApiError> {
    let pending_uploads: Vec<_> = repo
        .pending_uploads()
        .map_err(|_| UploadApiError::Unknown)?
        .iter()
        .map(UploadSummary::from)
        .collect();

    Ok(Json(PendingUploadsResponse {
        item_count: pending_uploads.len(),
        items: pending_uploads,
    }))
}
