use serde_json::{json, Value};

use crate::protocol::McpErrorBody;

pub fn handle_server_info(params: Option<Value>) -> Result<Value, McpErrorBody> {
    if params.is_some() {
        return Err(McpErrorBody::invalid_params(
            "server.info does not accept params",
        ));
    }

    Ok(json!({
        "name": "meganz-account-generator-mcp",
        "version": "0.1.0",
        "protocol_version": "mcp/1",
        "language": "rust",
        "transport": "stdio",
        "tools_supported": ["mega.generate"],
        "description": "MCP stdio server for generating Mega.nz accounts through a temporary email workflow"
    }))
}
