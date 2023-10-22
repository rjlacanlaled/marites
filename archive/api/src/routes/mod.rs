pub mod v1;

use crate::config;

use axum::{
    routing::{ MethodRouter, any_service },
    http::StatusCode,
    handler::HandlerWithoutStateExt,
};
use tower_http::services::ServeDir;

// constants
pub const AUTH_TOKEN: &str = "auth-token";

pub fn serve_dir() -> MethodRouter {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Resource not found.")
    }

    any_service(ServeDir::new(&config().WEB_FOLDER).not_found_service(handle_404.into_service()))
}
