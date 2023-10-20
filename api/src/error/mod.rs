use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub mod auth;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AuthError(auth::AuthError),
    InternalServerError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        print!("->> {:<12} - {self:?}", "INTO_RES");

        match self {
            Error::AuthError(e) => e.into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED ERROR").into_response(),
        }
    }
}
