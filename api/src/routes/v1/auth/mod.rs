use axum::{ response::{ Html, IntoResponse }, routing::get, Router };
use tracing::debug;

pub mod error;

pub fn routes_test_auth() -> Router {
    Router::new().route("/auth/test", get(test_auth_handler))
}

async fn test_auth_handler() -> impl IntoResponse {
    debug!("test_auth_handler");

    Html(format!("<strong>Hello, World!</strong><br><button>Go to Site</button>"))
}
