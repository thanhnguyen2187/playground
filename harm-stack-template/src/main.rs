mod templates;
mod db;
mod err;
mod schema;

use crate::templates::page_unimplemented;
use axum::{routing::get, Router};
use tower_http::services::ServeFile;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(page_unimplemented))
        .route_service("/styles.css", ServeFile::new("./static/styles.css"))
        .layer(LiveReloadLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
