use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct WebConf {
    #[clap(long, env, default_value = "0.0.0.0:3001")]
    pub listen: SocketAddr,
}

#[derive(Debug, Clone, Parser)]
pub struct JsonConf {
    #[clap(long)]
    pub body: PathBuf,

    /// Send once if set sleep to 0s.
    #[clap(long, default_value = "0s")]
    pub sleep: humantime::Duration,
}

#[derive(Debug, Clone, Parser)]
pub enum Mode {
    Json(JsonConf),
    Web(WebConf),
}

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[arg(long)]
    pub endpoint_url: String,

    #[arg(long)]
    pub targets: PathBuf,

    #[arg(long, default_value_t = 3)]
    pub retry_limit: usize,

    #[command(subcommand)]
    pub mode: Mode,
}
