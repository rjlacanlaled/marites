use anyhow::{Ok, Result};
// use serde_json::json;
use tokio::test;

#[test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/api/v1/test").await?.print().await?;

    Ok(())
}
