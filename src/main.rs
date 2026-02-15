mod app;
mod error;
mod handlers;
mod protocol;
mod state;

use std::error::Error as _;

use clap::Parser;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let cli = Cli::parse();
    let log_file = cli.log_file.filter(|value| !value.trim().is_empty());
    let proxy_url = cli.proxy_url.filter(|value| !value.trim().is_empty());

    if let Err(error) = app::run(log_file, proxy_url).await {
        eprintln!("error: {error}");
        let mut source = error.source();
        while let Some(cause) = source {
            eprintln!("caused by: {cause}");
            source = cause.source();
        }
        std::process::exit(1);
    }
}

#[derive(Parser, Debug)]
#[command(name = "meganz-account-generator-mcp")]
#[command(about = "MCP-style JSON-over-stdio server for generating MEGA.nz accounts")]
struct Cli {
    /// Proxy URL for MEGA requests
    #[arg(long = "proxy-url", env = "MEGA_PROXY_URL")]
    proxy_url: Option<String>,

    /// Log file path
    #[arg(long = "log-file", env = "MCP_LOG_FILE")]
    log_file: Option<String>,
}
