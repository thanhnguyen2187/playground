use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
// use kvs::Result;
use kvs::Error;
use std::sync::Arc;

// pub async fn get_() -> Json<String> {
// #[axum::debug_handler]
pub async fn get_(State(state): State<Arc<AppState>>) -> Result<Json<String>, Error> {
    Ok(Json(
        state
            .store
            .get("key1".to_owned())?
            .unwrap_or("Hello world".to_owned()),
    ))
}
