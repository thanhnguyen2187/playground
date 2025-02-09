mod db;
mod err;
mod schema;
mod templates;

use crate::err::Result;
use crate::templates::page_unimplemented;
use axum::{routing::get, Router};
use snafu::ResultExt;
use tower_http::services::ServeFile;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(page_unimplemented))
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
