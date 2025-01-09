use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use snafu::prelude::*;
pub use snafu::ResultExt;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("ID may not be less than 10, but it was {id}"))]
    InvalidId { id: u16 },

    #[snafu(display("Key {key} not found"))]
    KeyNotFound { key: String },

    #[snafu(display("Couldn't initialize file {path}"))]
    FileInit { path: String, err_str: String },

    #[snafu(display("Couldn't serialize command {command}"))]
    CommandSerialize { command: String, err_str: String },

    #[snafu(display("Couldn't write serialized command: {err_str}"))]
    CommandDeserialize { command: String, err_str: String },

    #[snafu(display("Couldn't write command as new line: {err_str}"))]
    CommandWriteLine { err_str: String },

    #[snafu(display("Couldn't open file {path}"))]
    FileOpen { path: String, err_str: String },

    #[snafu(display("Couldn't read content of file at {path}"))]
    FileRead { path: String, err_str: String },

    #[snafu(display("Couldn't write to file at {path}"))]
    FileWrite { path: String, err_str: String },

    #[snafu(whatever, display("{message}"))]
    Whatever {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },

    // IoError(std::io::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
