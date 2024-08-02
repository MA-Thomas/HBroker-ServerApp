use reqwest::Client;

pub async fn fetch_synthetic_data() -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get("https://health-data-bank-api.example/synthetic-data")
        .send()
        .await?;
    response.text().await
}