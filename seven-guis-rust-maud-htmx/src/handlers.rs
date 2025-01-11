use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn default_fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
