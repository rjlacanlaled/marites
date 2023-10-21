use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub fn routes_test() -> Router {
    Router::new().route("/test", get(test_handler))
}

async fn test_handler() -> impl IntoResponse {
    print!("->> {:<12} - {}", "TEST", "test_handler");

    Html(format!(
        "<strong>Hello, World!</strong><br><button>Go to Site</button>"
    ))
}
