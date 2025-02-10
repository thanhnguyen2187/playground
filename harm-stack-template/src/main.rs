mod db;
mod err;
mod schema;
mod templates;

use crate::db::MIGRATIONS;
use crate::err::{Error, Result};
use crate::templates::{home, page_toggle_todo, page_unimplemented};
use axum::{routing::get, Router};
use diesel::SqliteConnection;
use diesel_migrations::MigrationHarness;
use dotenvy::dotenv;
use snafu::ResultExt;
use std::sync::{Arc, Mutex};
use axum::routing::post;
use tower_http::services::ServeFile;
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
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|err| Error::DatabaseMigration {})?;
    // conn.run_pending_migrations(MIGRATIONS)
    //     .with_whatever_context(|err| format!("Failed to run migrations: {}", err))?;
    let app = Router::new()
        .route("/", get(home))
        .route("/toggle/{todo_id}", post(page_toggle_todo))
        .with_state(Arc::new(Mutex::new(AppState { conn })))
        .route_service("/styles.css", ServeFile::new("./static/styles.css"))
        .layer(LiveReloadLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .with_whatever_context(|err| format!("Failed to bind to port 3000: {}", err))?;
    axum::serve(listener, app)
        .await
        .with_whatever_context(|err| format!("Failed to serve: {}", err))?;

    Ok(())
}
