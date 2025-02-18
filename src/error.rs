use http::StatusCode;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    ParsingError,
    DatabaseError,
    Unauthorized,
    NotFound,
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::ParsingError => StatusCode::BAD_REQUEST,
            Error::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
