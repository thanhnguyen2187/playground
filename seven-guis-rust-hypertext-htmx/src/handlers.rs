use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn default_fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}