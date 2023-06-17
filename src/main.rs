use anyhow::{anyhow, Context, Result};
use log::{error, info};
use serde_json::Value;
use std::collections::HashMap;

async fn authorize(hostname: String, email: String, api: String) -> Result<String> {
    let client = reqwest::Client::new();

    let mut data = HashMap::new();
    data.insert("email", email);
    data.insert("api_key", api);

    let response = client
        .post(format!("https://{hostname}/v2api/auth/login"))
        .json(&data)
        .send()
        .await
        .context("Failed to send a request")?;

    if response.status().is_success() {
        let response_text = response
            .text()
            .await
            .context("Unable to get text from response")?;
        let parsed_response: Value =
            serde_json::from_str(&response_text).context("Unable to parse response as JSON")?;

        if let Some(token) = parsed_response.get("token") {
            info!("Authorization was successful");

            return Ok(token.to_string());
        } else {
            error!("Error: no `token` found in response: \n{}", response_text);
        }
    }

    Err(anyhow!("No token was found"))
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let hostname = std::env::var("HOSTNAME").context("Unable to retrieve `HOSTNAME` from env")?;
    let email = std::env::var("EMAIL").context("Unable to retrieve `EMAIL` from env")?;
    let api = std::env::var("API").context("Unable to retrieve `API` from env")?;

    println!("{}", authorize(hostname, email, api).await?);

    Ok(())
}
