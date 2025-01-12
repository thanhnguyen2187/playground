mod handlers;
mod index;
mod counter;
mod temperature_converter;
mod flight_booker;

use std::env;
use std::sync::{Arc, Mutex};
use axum::{routing::get, Router};
use axum::routing::post;
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
        .route("/", get(index::page))
        .route("/counter", get(counter::page))
        .route("/counter-increase", post(counter::page_increase))
        .route("/temperature-converter", get(temperature_converter::page))
        .route("/flight-booker", get(flight_booker::page))
        .route("/timer", get(index::page_unimplemented))
        .route("/crud", get(index::page_unimplemented))
        .route("/circle-drawer", get(index::page_unimplemented))
        .route("/hello-world", get(index::page_unimplemented))
        .fallback(handlers::default_fallback)
        .with_state(Arc::new(Mutex::new(AppState { counter: 0 })));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
