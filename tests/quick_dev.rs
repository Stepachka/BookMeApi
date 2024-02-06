
#[tokio::test]
async fn quick_dev() -> anyhow::Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    // client.do_get("/").await?.print().await?;
    // client.do_get("/api/post/test-post").await?.print().await?;
    // client.do_get("/api/user/test-user").await?.print().await?;
    client.do_post("/api/user/create-user", "").await?.print().await?;
    Ok(())
}