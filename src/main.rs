mod app;
mod error;
mod handlers;
mod protocol;
mod state;

use std::error::Error as _;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    if let Err(error) = app::run().await {
        eprintln!("error: {error}");
        let mut source = error.source();
        while let Some(cause) = source {
            eprintln!("caused by: {cause}");
            source = cause.source();
        }
        std::process::exit(1);
    }
}
