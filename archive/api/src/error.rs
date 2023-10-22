use crate::routes::v1::auth::error::AuthError;

use axum::{ http::StatusCode, response::{ IntoResponse, Response } };
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Config
    ConfigMissingEnv(&'static str),

    // Auth
    AuthError(AuthError),

    // Catch-all
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
