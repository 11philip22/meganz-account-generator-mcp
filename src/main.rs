mod handlers;
mod protocol;
mod state;

use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use serde_json::json;

use protocol::{McpErrorBody, McpRequest, McpResponse};
use state::AppState;

fn main() {
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(err) => {
            eprintln!("runtime init error: {err}");
            return;
        }
    };

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let app_state = AppState::default();
    let debug_enabled = std::env::var("MCP_DEBUG")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let mut log_file = match std::env::var("MCP_LOG_FILE") {
        Ok(path) => match OpenOptions::new().create(true).append(true).open(path) {
            Ok(file) => Some(file),
            Err(err) => {
                eprintln!("log file open error: {err}");
                None
            }
        },
        Err(_) => None,
    };

    for line in stdin.lock().lines() {
        let raw_line = match line {
            Ok(value) => value,
            Err(err) => {
                eprintln!("stdin read error: {err}");
                log_line(&mut log_file, &format!("stdin read error: {err}"));
                break;
            }
        };
        if debug_enabled {
            eprintln!("[mcp-debug] <= {raw_line}");
        }
        log_line(&mut log_file, &format!("[mcp] <= {raw_line}"));

        let response = match serde_json::from_str::<McpRequest>(&raw_line) {
            Ok(request) => runtime.block_on(dispatch_request(&app_state, request)),
            Err(_) => Some(McpResponse::err(
                json!("unknown"),
                McpErrorBody::invalid_request("malformed request JSON"),
            )),
        };

        let Some(response) = response else {
            if debug_enabled {
                eprintln!("[mcp-debug] => <no response>");
            }
            log_line(&mut log_file, "[mcp] => <no response>");
            continue;
        };

        let serialized = match serde_json::to_string(&response) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("response serialization error: {err}");
                log_line(
                    &mut log_file,
                    &format!("response serialization error: {err}"),
                );
                continue;
            }
        };
        if debug_enabled {
            eprintln!("[mcp-debug] => {serialized}");
        }
        log_line(&mut log_file, &format!("[mcp] => {serialized}"));

        if let Err(err) = writeln!(output, "{serialized}") {
            eprintln!("stdout write error: {err}");
            log_line(&mut log_file, &format!("stdout write error: {err}"));
            break;
        }
        if let Err(err) = output.flush() {
            eprintln!("stdout flush error: {err}");
            log_line(&mut log_file, &format!("stdout flush error: {err}"));
            break;
        }
    }
}

fn log_line(log_file: &mut Option<std::fs::File>, line: &str) {
    if let Some(file) = log_file.as_mut() {
        let _ = writeln!(file, "{line}");
        let _ = file.flush();
    }
}

async fn dispatch_request(state: &AppState, request: McpRequest) -> Option<McpResponse> {
    if request
        .jsonrpc
        .as_deref()
        .is_some_and(|version| version != "2.0")
    {
        return Some(McpResponse::err(
            request.id.unwrap_or(serde_json::Value::Null),
            McpErrorBody::invalid_request("jsonrpc must be 2.0"),
        ));
    }

    if request.id.is_none() {
        return None;
    }

    let id = request.id.unwrap_or(serde_json::Value::Null);

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
