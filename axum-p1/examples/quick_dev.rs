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
            "username":"kirami",
            "password":"password"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/api/tickets",
        json!(
            {
                "title": "Learn Rust"
            }
        ),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/api/tickets",
        json!(
            {
                "title": "Learn Rustlings"
            }
        ),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/api/tickets",
        json!(
            {
                "title": "Learn Rustaceans"
            }
        ),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/api/tickets").await?.print().await?;

    hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
