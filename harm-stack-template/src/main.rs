mod db;
mod err;
mod schema;
mod templates;

use crate::db::MIGRATIONS;
use crate::err::{Error, Result};
use crate::templates::{
    page_create_todo, page_default_todo, page_delete_todo, page_edit_todo, page_home,
    page_save_todo, page_toggle_todo, page_unimplemented,
};
use axum::routing::{delete, post};
use axum::{routing::get, Router};
use diesel::SqliteConnection;
use diesel_migrations::MigrationHarness;
use dotenvy::dotenv;
use snafu::ResultExt;
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

pub struct AppState {
    conn: SqliteConnection,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().with_whatever_context(|err| format!("Failed to load .env: {}", err))?;
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = db::establish_connection(&db_url)
        .with_whatever_context(|err| format!("Failed to connect to in-memory database {err}"))?;
    // TODO: resolve this issue in a more thorough way as the error is not very
    //       descriptive
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|_| Error::DatabaseMigration {})?;
    let app = Router::new()
        .route("/unimplemented", get(page_unimplemented))
        .route("/", get(page_home))
        .route("/toggle/{todo_id}", post(page_toggle_todo))
        .route("/default/{todo_id}", post(page_default_todo))
        .route("/edit/{todo_id}", post(page_edit_todo))
        .route("/save/{todo_id}", post(page_save_todo))
        .route("/create", post(page_create_todo))
        .route("/delete/{todo_id}", delete(page_delete_todo))
        .with_state(Arc::new(Mutex::new(AppState { conn })))
        .route_service("/{*wildcard}", ServeDir::new("./static"))
        .layer(LiveReloadLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .with_whatever_context(|err| format!("Failed to bind to port 3000: {}", err))?;
    axum::serve(listener, app)
        .await
        .with_whatever_context(|err| format!("Failed to serve: {}", err))?;

    Ok(())
}
