mod uploads;

use axum::Router;

pub fn get_api_routes() -> Router<()> {
    let v1_router = get_v1_router();

    Router::new().nest("/v1", v1_router)
}

fn get_v1_router() -> Router<()> {
    let uploads_router = uploads::router();

    Router::new().nest("/uploads", uploads_router)
}
