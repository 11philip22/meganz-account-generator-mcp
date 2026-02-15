use serde_json::{Value, json};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

use crate::error::Error;
use crate::handlers;
use crate::protocol::{McpErrorBody, McpRequest, McpResponse};
use crate::state::AppState;

pub async fn run(log_file: Option<String>, proxy_url: Option<String>) -> Result<(), Error> {
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();
    let mut stdout = io::stdout();
    let app_state = AppState::new(proxy_url);

    init_tracing(log_file.clone());

    while let Some(raw_line) = lines.next_line().await.map_err(Error::ReadStdin)? {
        info!("[mcp] <= {raw_line}");

        let response = match serde_json::from_str::<McpRequest>(&raw_line) {
            Ok(request) => dispatch_request(&app_state, request).await,
            Err(_) => Some(McpResponse::err(
                json!("unknown"),
                McpErrorBody::invalid_request("malformed request JSON"),
            )),
        };

        let Some(response) = response else {
            info!("[mcp] => <no response>");
            continue;
        };

        let serialized = serde_json::to_string(&response).map_err(Error::SerializeResponse)?;
        info!("[mcp] => {serialized}");

        stdout
            .write_all(serialized.as_bytes())
            .await
            .map_err(Error::WriteStdout)?;
        stdout.write_all(b"\n").await.map_err(Error::WriteStdout)?;
        stdout.flush().await.map_err(Error::FlushStdout)?;
    }

    Ok(())
}

pub fn init_tracing(log_file: Option<String>) {
    if let Some(path) = log_file {  
        let file_appender =
            tracing_appender::rolling::never(".", path);
        fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_writer(file_appender)
            .without_time()
            .with_level(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(false)
            .init();
    } else {
        fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init(); // default stderr formatting
    }
}

async fn dispatch_request(state: &AppState, request: McpRequest) -> Option<McpResponse> {
    if request
        .jsonrpc
        .as_deref()
        .is_some_and(|version| version != "2.0")
    {
        return Some(McpResponse::err(
            request.id.unwrap_or(Value::Null),
            McpErrorBody::invalid_request("jsonrpc must be 2.0"),
        ));
    }

    if request.id.is_none() {
        return None;
    }

    let id = request.id.unwrap_or(Value::Null);

    if !id.is_string() && !id.is_number() {
        return Some(McpResponse::err(
            id,
            McpErrorBody::invalid_request("id must be a string or number"),
        ));
    }

    if request.method.trim().is_empty() {
        return Some(McpResponse::err(
            id,
            McpErrorBody::invalid_request("method is required"),
        ));
    }

    match request.method.as_str() {
        "initialize" => match handlers::handle_initialize(request.params) {
            Ok(result) => Some(McpResponse::ok(id, result)),
            Err(error) => Some(McpResponse::err(id, error)),
        },
        "tools/list" => Some(McpResponse::ok(id, handlers::handle_tools_list(state))),
        "tools/call" => match handlers::handle_tool_call(state, request.params).await {
            Ok(result) => Some(McpResponse::ok(id, result)),
            Err(error) => Some(McpResponse::err(id, error)),
        },
        _ => Some(McpResponse::err(
            id,
            McpErrorBody::method_not_found("unknown method"),
        )),
    }
}
