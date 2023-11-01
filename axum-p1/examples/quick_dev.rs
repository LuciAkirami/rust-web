#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=Luci").await?.print().await?;
    hc.do_get("/hello/Lucifer").await?.print().await?;
    hc.do_post(
        "/api/login",
        json!({
            "username":"akirami",
            "password":"password"
        }),
    )
    .await?
    .print()
    .await?;
    Ok(())
}
