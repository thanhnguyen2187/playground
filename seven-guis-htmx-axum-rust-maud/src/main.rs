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
use log::info;
use maud::{html, Markup};
use crate::common::header;
use crate::crud::{state_mod as state_crud};
use crate::flight_booker::{FlightBookerState, OneWayFlight};

#[derive(Debug)]
pub struct AppState {
    counter: i32,
    flight_booker_state: FlightBookerState,
    crud_state: state_crud::Impl,
}

pub async fn page() -> Markup {
    html! {
        (header("Seven GUIs in Rust"))
        body hx-boost="true" {
            h1 { "Seven GUIs in Rust" }
            ul {
                li { a href="/counter" { "Counter" } }
                li { a href="/temperature-converter" { "Temperature Converter" } }
                li { a href="/flight-booker" { "Flight Booker" } }
                li { a href="/timer" { "Timer" } }
                li { a href="/crud" { "CRUD" } }
                li { a href="/circle-drawer" { "Circle Drawer (Unimplemented)" } }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    info!("Logger initialized!");
    info!("Logger level: {}", env::var("RUST_LOG").unwrap_or("debug".to_string()));

    let app = Router::new()
        .route("/", get(page))
        .route("/counter", get(counter::page))
        .route("/counter-increase", post(counter::page_increase))
        .route("/temperature-converter", get(temperature_converter::page))
        .route("/flight-booker", get(flight_booker::page))
        .route("/flight-booker-submit", post(flight_booker::page_submit))
        .route("/timer", get(timer::page))
        .route("/crud", get(crud::page))
        .route("/crud-state/{field}", post(crud::mutate_state))
        .route("/crud", post(crud::create))
        .route("/crud", put(crud::update))
        .route("/crud", delete(crud::delete))
        .route("/crud/update-filter", post(crud::update_filter))
        .route("/circle-drawer", get(common::page_unimplemented))
        .route("/hello-world", get(common::page_unimplemented))
        .fallback(handlers::default_fallback)
        .with_state(Arc::new(Mutex::new(AppState {
            counter: 0,
            flight_booker_state:
            FlightBookerState::OneWay(
                OneWayFlight {
                    from: None,
                }
            ),
            crud_state: state_crud::new(),
        })));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
