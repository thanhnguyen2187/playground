use std::sync::{Arc, RwLock};
use kvs::KvsEngine;

#[derive(Clone)]
pub struct AppState {
    // TODO: use dashmap (https://docs.rs/dashmap/latest/dashmap/struct.DashMap.html)
    //       to make it thread-safe instead of hand-rolling it
    pub store: Arc<RwLock<dyn KvsEngine>>,
}
