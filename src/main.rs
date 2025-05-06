use clap::Parser;
use std::fs;

mod conf;
mod json;
mod req;
mod web;

use conf::{Cli, Mode};
use req::{ENV, Env};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let targets = fs::read_to_string(&cli.targets).expect("Failed to open targets file");

    let targets: Vec<String> = targets
        .split('\n')
        .filter_map(|line| {
            line.split(' ')
                .last()
                .map(|v| v.to_string())
                .filter(|s| !String::is_empty(s))
                .map(|v| format!("{v}?wait=true"))
        })
        .collect();

    println!("{} target(s) detected!", targets.len());

    ENV.set(Env {
        token: cli.token,
        targets,
        endpoint_url: cli.endpoint_url,
        retry_limit: cli.retry_limit,
    })
    .unwrap();

    match cli.mode {
        Mode::Json(conf) => {
            json::run(conf.body, conf.sleep.into()).await;
        }
        Mode::Web(conf) => {
            web::run(conf.listen).await;
        }
    }
}
