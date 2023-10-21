use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        print!("->> {:<12} - {self:?}", "INTO_RES");

        match self {
            AuthError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Please provide a valid token.").into_response()
            }
        }
    }
}
