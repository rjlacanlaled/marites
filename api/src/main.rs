use std::net::SocketAddr;

pub use self::error::{Error, Result};
use crate::routers::routes_static;
use crate::routers::v1::test::routes_test;

use axum::Router;

mod error;
mod routers;

#[tokio::main]
async fn main() -> Result<()> {
    let routes_all = Router::new()
        .merge(routes_test())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
