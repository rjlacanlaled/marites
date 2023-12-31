mod config;
mod error;
mod middlewares;
mod routes;
mod model;
mod context;

pub mod _dev_utils;

pub use self::error::{ Error, Result };
pub use config::config;

use crate::routes::serve_dir;
use crate::routes::v1::auth::routes_test_auth;
use crate::routes::v1::test::routes_test;
use crate::middlewares::auth_middleware::with_auth;

use std::net::SocketAddr;
use axum::{ middleware, Router };
use tower_cookies::CookieManagerLayer;
use tracing::{ info, debug };
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn start() -> Result<()> {
    tracing_subscriber
        ::fmt()
        .without_time() // For testing only
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // For dev only
    _dev_utils::init_dev().await;

    let routes_protected = routes_test().route_layer(middleware::from_fn(with_auth));
    let routes_public = routes_test_auth();

    let routes_all = Router::new()
        .nest("/api/v1", routes_protected)
        .nest("/api/v1", routes_public)
        .layer(CookieManagerLayer::new())
        .fallback_service(serve_dir());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(routes_all.into_make_service()).await.unwrap();

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        debug!("Error: {:?}", err);
    }
}
