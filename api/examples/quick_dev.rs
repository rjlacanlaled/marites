use anyhow::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/api/v1/test").await?.print().await?;

    Ok(())
}
