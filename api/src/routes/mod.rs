pub mod v1;

use axum::{ routing::get_service, Router };
use tower_http::services::ServeDir;

// constants
pub const AUTH_TOKEN: &str = "auth-token";

pub fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("/")))
}
