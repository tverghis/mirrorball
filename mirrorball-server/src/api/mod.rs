mod uploads;

use axum::Router;

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
