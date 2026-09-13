use std::{
    fs::File,
    num::NonZeroU64,
    path::{Path, PathBuf},
};

use axum::{Json, extract::State};
use mirrorball_api::{
    CreateUploadRequest, CreateUploadResponse, PendingUploadsResponse, UploadSummary,
};

use super::domain::*;
use crate::{
    api::{ApiResponse, ApiState},
    common::ChunkDigest,
};

pub async fn create_upload_request(
    State(state): State<ApiState>,
    Json(body): Json<CreateUploadRequest>,
) -> ApiResponse<CreateUploadResponse> {
    let num_chunk_hashes = body.chunk_hashes.len();

    if !(1..=256).contains(&num_chunk_hashes) {
        return Err(UploadApiError::ChunkHashCount(num_chunk_hashes).into());
    }

    let digests: Result<Vec<_>, anyhow::Error> = body
        .chunk_hashes
        .iter()
        .map(|s| ChunkDigest::try_from(s.as_str()))
        .collect();

    let digests = digests.map_err(|_| UploadApiError::InvalidDigest)?;

    let destination_path = PathBuf::from(body.destination);

    let upload = state
        .repository
        .new_upload(destination_path, body.file_size, &digests)
        .map_err(|_| UploadApiError::CreateUpload)?;

    let staging_path = state.staging.join(&upload.token);

    create_sparse_file(&staging_path, body.file_size)
        .map_err(|_| UploadApiError::CreateStagingFile)?;

    Ok(Json(CreateUploadResponse {
        token: upload.token,
    }))
}

// Pre-allocates the staging file to the full upload size so chunks can be
// written at arbitrary offsets without the file growing or moving.
fn create_sparse_file(path: &Path, size: NonZeroU64) -> std::io::Result<()> {
    let file = File::create(path)?;
    file.set_len(size.get())?;

    Ok(())
}

pub async fn get_pending_uploads(
    State(state): State<ApiState>,
) -> ApiResponse<PendingUploadsResponse> {
    let pending_uploads: Vec<_> = state
        .repository
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
