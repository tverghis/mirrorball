mod api;
mod models;
mod repository;

use axum::Router;

use crate::api::get_api_routes;

#[tokio::main]
async fn main() {
    let api_router = get_api_routes();

    let app = Router::new().nest("/api", api_router);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
