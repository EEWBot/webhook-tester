use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub async fn run(body: PathBuf, sleep: Duration) {
    let body = fs::read_to_string(&body).expect("Failed to open targets file");
    let body: serde_json::Value = serde_json::from_str(&body).expect("Failed to read as JSON");

    loop {
        println!(
            "Response: {}",
            crate::req::req(body.clone())
                .await
                .expect("Failed to request")
        );

        if sleep.is_zero() {
            break;
        }

        tokio::time::sleep(sleep).await;
    }
}
