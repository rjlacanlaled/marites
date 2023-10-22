use std::env;

use tracing::debug;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // Setup tracing
    env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();
    tracing_subscriber
        ::fmt()
        .without_time() // For testing only
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        debug!("Error: {:?}", err);
    }
}
