use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

pub mod v1;

pub fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("/")))
}
