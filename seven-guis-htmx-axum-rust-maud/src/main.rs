mod handlers;
mod common;
mod counter;
mod temperature_converter;
mod flight_booker;
mod timer;
mod crud;
mod err;
mod db;
mod schema;

use std::env;
use std::sync::{Arc, Mutex};
use axum::{routing::get, Router};
use axum::routing::{delete, post, put};
use diesel::SqliteConnection;
use dotenvy::dotenv;
use log::info;
use maud::{html, Markup};
use crate::common::header;
use crate::db::establish_connection;

pub struct AppState {
    counter: i32,
    sqlite_connection: SqliteConnection,
}

pub async fn page() -> Markup {
    html! {
        (header("7 GUIs in HARM Stack"))
        body hx-boost="true" {
            h1 { "7 GUIs in HARM Stack" }
            ul {
                li { a href="/counter" { "Counter" } }
                li { a href="/temperature-converter" { "Temperature Converter" } }
                li { a href="/flight-booker" { "Flight Booker" } }
                li { a href="/timer" { "Timer" } }
                li { a href="/crud" { "CRUD" } }
                li { a href="/circle-drawer" { "Circle Drawer (Unimplemented)" } }
                li { a href="/spreadsheet" { "Spreadsheet (Unimplemented)" } }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    info!("Logger initialized!");
    info!("Logger level: {}", env::var("RUST_LOG").unwrap_or("debug".to_string()));

    let app = Router::new()
        .route("/", get(page))
        .route("/counter", get(counter::page))
        .route("/counter-increase", post(counter::increase))
        .route("/temperature-converter", get(temperature_converter::page))
        .route("/flight-booker", get(flight_booker::page))
        .route("/flight-booker-submit", post(flight_booker::page_submit))
        .route("/timer", get(timer::page))
        .route("/crud", get(crud::page))
        .route("/crud", post(crud::create))
        .route("/crud", put(crud::update))
        .route("/crud", delete(crud::delete))
        .route("/crud/update-filter", post(crud::update_filter))
        .route("/circle-drawer", get(common::page_unimplemented))
        .route("/spreadsheet", get(common::page_unimplemented))
        .fallback(handlers::default_fallback)
        .with_state(Arc::new(Mutex::new(AppState {
            counter: 0,
            sqlite_connection: establish_connection().expect("Failed to connect to database"),
        })));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
