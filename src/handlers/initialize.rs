use serde::Deserialize;
use serde_json::{json, Value};

use crate::protocol::McpErrorBody;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    protocol_version: String,
    #[serde(rename = "clientInfo")]
    client_info: Value,
    capabilities: Value,
}

pub fn handle_initialize(params: Option<Value>) -> Result<Value, McpErrorBody> {
    let params_value = params.ok_or_else(|| {
        McpErrorBody::invalid_params("initialize requires params object")
    })?;
    let init: InitializeParams = serde_json::from_value(params_value)
        .map_err(|_| McpErrorBody::invalid_params("invalid initialize params shape"))?;

    if !init.client_info.is_object() || !init.capabilities.is_object() {
        return Err(McpErrorBody::invalid_params(
            "invalid initialize params shape",
        ));
    }

    Ok(json!({
        "protocolVersion": init.protocol_version,
        "serverInfo": {
            "name": "meganz-account-generator-mcp",
            "version": "0.1.0"
        },
        "capabilities": {
            "tools": {}
        }
    }))
}
