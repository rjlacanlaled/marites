use axum::{ http::StatusCode, response::{ IntoResponse, Response } };
use tracing::debug;

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        debug!("{} - {}", "AUTH_ERROR", "into_response");

        match self {
            AuthError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Please provide a valid token.").into_response()
            }
        }
    }
}
