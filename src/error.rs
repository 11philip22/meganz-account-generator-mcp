use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read from stdin")]
    ReadStdin(#[source] std::io::Error),
    #[error("failed to write to stdout")]
    WriteStdout(#[source] std::io::Error),
    #[error("failed to flush stdout")]
    FlushStdout(#[source] std::io::Error),
    #[error("failed to serialize MCP response")]
    SerializeResponse(#[source] serde_json::Error),
}
