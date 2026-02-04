mod handlers;
mod protocol;
mod state;

use std::io::{self, BufRead, Write};

use protocol::{McpErrorBody, McpRequest, McpResponse};
use serde_json::json;
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

    for line in stdin.lock().lines() {
        let raw_line = match line {
            Ok(value) => value,
            Err(err) => {
                eprintln!("stdin read error: {err}");
                break;
            }
        };

        let response = match serde_json::from_str::<McpRequest>(&raw_line) {
            Ok(request) => runtime.block_on(dispatch_request(&app_state, request)),
            Err(_) => McpResponse::err(
                json!("unknown"),
                McpErrorBody::invalid_request("malformed request JSON"),
            ),
        };

        let serialized = match serde_json::to_string(&response) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("response serialization error: {err}");
                continue;
            }
        };

        if let Err(err) = writeln!(output, "{serialized}") {
            eprintln!("stdout write error: {err}");
            break;
        }
        if let Err(err) = output.flush() {
            eprintln!("stdout flush error: {err}");
            break;
        }
    }
}

async fn dispatch_request(state: &AppState, request: McpRequest) -> McpResponse {
    if !request.id.is_string() && !request.id.is_number() {
        return McpResponse::err(
            request.id,
            McpErrorBody::invalid_request("id must be a string or number"),
        );
    }

    if request.jsonrpc.as_deref().is_some_and(|version| version != "2.0") {
        return McpResponse::err(
            request.id,
            McpErrorBody::invalid_request("jsonrpc must be 2.0"),
        );
    }

    if request.method.trim().is_empty() {
        return McpResponse::err(
            request.id,
            McpErrorBody::invalid_request("method is required"),
        );
    }

    match request.method.as_str() {
        "initialize" => match handlers::handle_initialize(request.params) {
            Ok(result) => McpResponse::ok(request.id, result),
            Err(error) => McpResponse::err(request.id, error),
        },
        "server/info" => match handlers::handle_server_info(request.params) {
            Ok(result) => McpResponse::ok(request.id, result),
            Err(error) => McpResponse::err(request.id, error),
        },
        "tools/list" => {
            McpResponse::ok(request.id, handlers::handle_tools_list(state))
        }
        "mega/generate" => match handlers::handle_generate(state, request.params).await {
            Ok(result) => McpResponse::ok(request.id, result),
            Err(error) => McpResponse::err(request.id, error),
        },
        _ => McpResponse::err(
            request.id,
            McpErrorBody::method_not_found("unknown method"),
        ),
    }
}
