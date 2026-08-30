use std::{path::PathBuf, sync::Arc};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::Upload, repository::UploadsRepository};

pub fn router(repository: Arc<dyn UploadsRepository>) -> Router<()> {
    Router::new()
        .route("/upload_request", post(create_upload_request))
        .route("/pending", get(get_pending_uploads))
        .with_state(repository)
}

#[derive(Debug)]
enum UploadApiError {
    CreateUpload,
    Unknown,
}

impl IntoResponse for UploadApiError {
    fn into_response(self) -> Response {
        match self {
            Self::CreateUpload => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create upload"),
            Self::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An unknown error occurred",
            ),
        }
        .into_response()
    }
}

#[derive(Debug, Deserialize)]
struct CreateUploadRequest {
    file_size: usize,
    destination: PathBuf,
}

#[derive(Debug, Serialize)]
struct CreateUploadResponse {
    #[serde(with = "uuid::serde::hyphenated")]
    token: Uuid,
}

async fn create_upload_request(
    State(repo): State<Arc<dyn UploadsRepository>>,
    Json(body): Json<CreateUploadRequest>,
) -> Result<Json<CreateUploadResponse>, UploadApiError> {
    let token = Uuid::new_v4();

    repo.new_upload(token, body.destination, body.file_size, 0)
        .map_err(|_| UploadApiError::CreateUpload)?;

    Ok(Json(CreateUploadResponse { token }))
}

#[derive(Debug, Serialize)]
struct PendingUploadsResponse {
    item_count: usize,
    items: Vec<Upload>,
}

async fn get_pending_uploads(
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
