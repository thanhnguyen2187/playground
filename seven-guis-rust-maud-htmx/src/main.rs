mod handlers;
mod templates;

use std::env;
use axum::{routing::get, Router};
use maud::Markup;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    info!("Logger initialized!");
    info!("Logger level: {}", env::var("RUST_LOG").unwrap_or("debug".to_string()));

    let app = Router::new()
        .route("/", get(templates::index))
        .route("/counter", get(templates::counter))
        .route("/temperature-converter", get(templates::unimplemented))
        .route("/flight-booker", get(templates::unimplemented))
        .route("/timer", get(templates::unimplemented))
        .route("/crud", get(templates::unimplemented))
        .route("/circle-drawer", get(templates::unimplemented))
        .route("/hello-world", get(templates::unimplemented))
        .fallback(handlers::default_fallback);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
