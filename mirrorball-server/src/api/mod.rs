mod uploads;

use axum::{
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use mirrorball_api::ErrorResponse;

use crate::{config::Config, repository};

pub fn get_api_routes(config: &Config) -> Router<()> {
    let v1_router = get_v1_router(config);

    Router::new().nest("/v1", v1_router)
}

fn get_v1_router(config: &Config) -> Router<()> {
    let repo = repository::for_config(config);

    let uploads_router = uploads::router(repo);

    Router::new().nest("/uploads", uploads_router)
}

pub type ApiResponse<T> = Result<Json<T>, ApiError>;

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorResponse {
                code: self.code.to_string(),
                message: self.message,
            }),
        )
            .into_response()
    }
}
