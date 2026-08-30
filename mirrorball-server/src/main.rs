mod api;
mod common;
mod config;
mod models;
mod repository;
mod utils;

use axum::Router;

use crate::{api::get_api_routes, config::Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_file("mirrorball.toml").unwrap();

    ensure_data_root(&config)?;

    let api_router = get_api_routes(&config);

    let app = Router::new().nest("/api", api_router);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}

fn ensure_data_root(config: &Config) -> std::io::Result<()> {
    std::fs::create_dir_all(&config.root)?;

    Ok(())
}
