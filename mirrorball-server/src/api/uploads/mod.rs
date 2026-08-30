mod domain;
mod handlers;

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::repository::UploadsRepository;

use handlers::*;

pub fn router(repository: Arc<dyn UploadsRepository>) -> Router<()> {
    Router::new()
        .route("/upload_request", post(create_upload_request))
        .route("/pending", get(get_pending_uploads))
        .with_state(repository)
}
