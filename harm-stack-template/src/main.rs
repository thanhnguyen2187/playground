mod templates;

use crate::templates::page_unimplemented;
use axum::{routing::get, Router};
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(page_unimplemented))
        .route_service("/styles.css", ServeFile::new("./static/styles.css"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
