use http::StatusCode;
use leptos::server_fn::{
    codec::JsonEncoding,
    error::{FromServerFnError, ServerFnErrorErr},
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Error {
    Io(String),
    Db(String),

    Unauthorized,
    NotFound,

    InternalError,

    // internal Leptos server-fn machinery
    ServerFnError(ServerFnErrorErr),
}

impl core::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let msg = match self {
            Error::Io(_) => "I/O error",
            Error::Db(_) => "Database error",
            Error::Unauthorized => "Unauthorized",
            Error::NotFound => "Not found",
            Error::InternalError => "Internal error",
            Error::ServerFnError(e) => return write!(f, "{e}"),
        };
        write!(f, "{msg}")
    }
}

impl FromServerFnError for Error {
    type Encoder = JsonEncoding;
    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Error::ServerFnError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Db(e.to_string())
    }
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            // Error::Parse(_) => StatusCode::BAD_REQUEST,
            Error::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // Error::Http(_) => StatusCode::BAD_GATEWAY, // or 502/500 to taste
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ServerFnError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "ssr")]
impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (self.status_code(), self.to_string()).into_response()
    }
}

