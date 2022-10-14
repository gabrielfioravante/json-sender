use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()>  {
    json_sender::init().await?;
    Ok(())
}
