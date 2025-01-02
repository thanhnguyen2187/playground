use crate::AppState;
use axum::extract::{Path, State};
use std::ops::{Deref, DerefMut};
// use axum::response::IntoResponse;
use axum::Json;
use kvs::{Error, Result};
use snafu::whatever;
use std::sync::{Arc, Mutex};
use log::{info, warn};

pub async fn get(State(state): State<AppState>, Path(key): Path<String>) -> Result<Json<String>> {
    println!("key: {:?}", key);
    if let Ok(state_lock) = state.store.lock() {
        let state = state_lock.deref();
        let value_opt = state.get(key.clone())?;
        if let Some(value) = value_opt {
            info!("Found value for key {}", key);
            Ok(Json(value))
        } else {
            warn!("Couldn't find value for key {}", key);
            Ok(Json("Not found".to_owned()))
        }
    } else {
        whatever!("Unable to acquire write lock on state");
    }
}

pub async fn set(
    State(state): State<AppState>,
    Path((key, value)): Path<(String, String)>,
) -> Result<&'static str> {
    if let Ok(mut state_lock) = state.store.lock() {
        let state = state_lock.deref_mut();
        state.set(key, value.to_owned())?;
        Ok("Success!")
    } else {
        whatever!("Unable to acquire write lock on state");
    }
}
