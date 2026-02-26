mod app;
mod error;
mod handlers;
mod protocol;
mod state;
use std::error::Error as _;
use tracing_subscriber::{EnvFilter, fmt};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "meganz-account-generator-mcp")]
#[command(about = "MCP-style JSON-over-stdio server for generating MEGA.nz accounts")]
struct Cli {
    /// Proxy URL for MEGA requests
    #[arg(long = "proxy-url", env = "MEGA_PROXY_URL")]
    proxy_url: Option<String>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    init_tracing();

    let cli = Cli::parse();
    let proxy_url = cli.proxy_url.filter(|value| !value.trim().is_empty());

    if let Err(error) = app::run(proxy_url).await {
        eprintln!("error: {error}");
        let mut source = error.source();
        while let Some(cause) = source {
            eprintln!("caused by: {cause}");
            source = cause.source();
        }
        std::process::exit(1);
    }
}

pub fn init_tracing() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .with_level(false)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_ansi(false)
        .init();
}
