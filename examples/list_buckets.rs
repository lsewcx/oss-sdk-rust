use oss_sdk_rust::Oss;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oss = Oss::new(
        std::env::var("OSS_ACCESS_KEY_ID")?,
        std::env::var("OSS_ACCESS_KEY_SECRET")?,
        std::env::var("OSS_REGION").unwrap_or_else(|_| "cn-hangzhou".to_string()),
    );

    let resp = oss.list_buckets().await?;
    println!("status: {}", resp.status());
    println!("{}", resp.text().await?);
    Ok(())
}
