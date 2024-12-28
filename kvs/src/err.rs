use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("ID may not be less than 10, but it was {id}"))]
    InvalidId { id: u16 },

    #[snafu(display("Key {key} not found"))]
    KeyNotFound { key: String },
}

pub type Result<T> = std::result::Result<T, Error>;
