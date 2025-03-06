use reqwest::Client;
use std::{fs::File, io::Read};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    File::open("./key/ca.pem")?.read_to_end(&mut buf)?;
    let cert = reqwest::Certificate::from_pem(&buf)?;
    // 建立 reqwest 客戶端
    let client = Client::builder().add_root_certificate(cert).build()?;

    // 連線到 HTTPS 伺服器
    let res = client.get("https://localhost:8080/ping").send().await?;
    let body = res.text().await?;
    println!("Response: {}", body);

    Ok(())
}
