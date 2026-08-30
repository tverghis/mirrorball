mod uploads;

use std::sync::Arc;

use axum::Router;

use crate::repository::InMemoryRepository;

pub fn get_api_routes() -> Router<()> {
    let v1_router = get_v1_router();

    Router::new().nest("/v1", v1_router)
}

fn get_v1_router() -> Router<()> {
    let user_repo = Arc::new(InMemoryRepository::default());
    let uploads_router = uploads::router(user_repo);

    Router::new().nest("/uploads", uploads_router)
}
