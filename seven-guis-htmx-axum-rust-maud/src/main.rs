mod handlers;
mod templates;

use std::env;
use std::sync::{Arc, Mutex};
use axum::{routing::get, Router};
use axum::routing::post;
use maud::Markup;
use log::info;

pub struct AppState {
    counter: i32,
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    info!("Logger initialized!");
    info!("Logger level: {}", env::var("RUST_LOG").unwrap_or("debug".to_string()));

    let app = Router::new()
        .route("/", get(templates::page_index))
        .route("/counter", get(templates::page_counter))
        .route("/counter-increase", post(templates::page_counter_increase))
        .route("/temperature-converter", get(templates::page_unimplemented))
        .route("/flight-booker", get(templates::page_unimplemented))
        .route("/timer", get(templates::page_unimplemented))
        .route("/crud", get(templates::page_unimplemented))
        .route("/circle-drawer", get(templates::page_unimplemented))
        .route("/hello-world", get(templates::page_unimplemented))
        .fallback(handlers::default_fallback)
        .with_state(Arc::new(Mutex::new(AppState { counter: 0 })));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
