use std::env;

use serde_json::{Value, json};
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::error::Error;
use crate::handlers;
use crate::protocol::{McpErrorBody, McpRequest, McpResponse};
use crate::state::AppState;

pub async fn run() -> Result<(), Error> {
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();
    let mut stdout = io::stdout();
    let app_state = AppState::default();
    let debug_enabled = debug_enabled();
    let mut log_file = open_log_file().await;

    while let Some(raw_line) = lines.next_line().await.map_err(Error::ReadStdin)? {
        if debug_enabled {
            eprintln!("[mcp-debug] <= {raw_line}");
        }
        log_line(&mut log_file, &format!("[mcp] <= {raw_line}")).await;

        let response = match serde_json::from_str::<McpRequest>(&raw_line) {
            Ok(request) => dispatch_request(&app_state, request).await,
            Err(_) => Some(McpResponse::err(
                json!("unknown"),
                McpErrorBody::invalid_request("malformed request JSON"),
            )),
        };

        let Some(response) = response else {
            if debug_enabled {
                eprintln!("[mcp-debug] => <no response>");
            }
            log_line(&mut log_file, "[mcp] => <no response>").await;
            continue;
        };

        let serialized = serde_json::to_string(&response).map_err(Error::SerializeResponse)?;
        if debug_enabled {
            eprintln!("[mcp-debug] => {serialized}");
        }
        log_line(&mut log_file, &format!("[mcp] => {serialized}")).await;

        stdout
            .write_all(serialized.as_bytes())
            .await
            .map_err(Error::WriteStdout)?;
        stdout.write_all(b"\n").await.map_err(Error::WriteStdout)?;
        stdout.flush().await.map_err(Error::FlushStdout)?;
    }

    Ok(())
}

async fn open_log_file() -> Option<File> {
    let path = match env::var("MCP_LOG_FILE") {
        Ok(path) => path,
        Err(_) => return None,
    };

    match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .await
    {
        Ok(file) => Some(file),
        Err(err) => {
            eprintln!("log file open error: {err}");
            None
        }
    }
}

fn debug_enabled() -> bool {
    env::var("MCP_DEBUG")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

async fn log_line(log_file: &mut Option<File>, line: &str) {
    let Some(file) = log_file.as_mut() else {
        return;
    };

    if file.write_all(line.as_bytes()).await.is_err() {
        return;
    }
    if file.write_all(b"\n").await.is_err() {
        return;
    }
    let _ = file.flush().await;
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
