use axum::{ http::StatusCode, response::{ IntoResponse, Response } };
use tracing::debug;

use crate::routes::v1::auth::error::AuthError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AuthError(AuthError),
    InternalServerError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{} - {}", "ERROR", "into_response");

        match self {
            Error::AuthError(e) => e.into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED ERROR").into_response(),
        }
    }
}
