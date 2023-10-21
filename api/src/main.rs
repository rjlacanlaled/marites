use std::net::SocketAddr;

pub use self::error::{Error, Result};
use crate::routes::v1::auth::routes_test_auth;
use crate::routes::v1::test::routes_test;
use crate::{middlewares::auth_middleware::with_auth, routes::routes_static};

use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;

mod error;
mod middlewares;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let routes_protected = routes_test().route_layer(middleware::from_fn(with_auth));
    let routes_public = routes_test_auth();

    let routes_all = Router::new()
        .nest("/api/v1", routes_protected)
        .nest("/api/v1", routes_public)
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
