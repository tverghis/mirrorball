mod domain;
mod handlers;

use axum::{
    Router,
    routing::{get, post},
};

use crate::api::ApiState;

use handlers::*;

pub fn router(state: ApiState) -> Router<()> {
    Router::new()
        .route("/upload_request", post(create_upload_request))
        .route("/pending", get(get_pending_uploads))
        .with_state(state)
}
