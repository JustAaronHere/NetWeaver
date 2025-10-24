use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    netweaver_lib::run().await
}
