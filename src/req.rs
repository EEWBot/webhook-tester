use anyhow::{Context, Result};
use serde::Serialize;
use once_cell::sync::OnceCell;

pub static ENV: OnceCell<Env> = OnceCell::new();

#[derive(Debug)]
pub struct Env {
    pub targets: Vec<String>,
    pub endpoint_url: String,
    pub retry_limit: usize,
}

#[derive(Debug, Serialize)]
struct Request {
    targets: &'static [String],
    body: serde_json::Value,
    retry_limit: usize,
}

pub async fn req(body: serde_json::Value) -> Result<String> {
    let env = ENV.get().context("Build Error")?;

    let client = reqwest::ClientBuilder::new()
        .user_agent("WebhookTester/0.1.0")
        .build()
        .unwrap();

    client
        .post(&env.endpoint_url)
        .json(&Request {
            body,
            targets: &env.targets,
            retry_limit: env.retry_limit,
        })
        .send()
        .await
        .context("Failed to send request to server")?
        .error_for_status()
        .context("Error response")?
        .text()
        .await
        .context("Failed to get response body")
}
