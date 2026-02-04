use serde::Deserialize;
use serde_json::{Value, json};

use crate::protocol::McpErrorBody;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ToolCallParams {
    #[serde(rename = "_meta")]
    _meta: Option<Value>,
    name: Option<String>,
    arguments: Option<Value>,
}

pub async fn handle_tool_call(
    state: &AppState,
    params: Option<Value>,
) -> Result<Value, McpErrorBody> {
    let params =
        params.ok_or_else(|| McpErrorBody::invalid_params("tools/call requires params object"))?;

    let parsed: ToolCallParams = serde_json::from_value(params)
        .map_err(|err| McpErrorBody::invalid_params(format!("invalid tools/call params: {err}")))?;

    let tool_name = parsed
        .name
        .ok_or_else(|| McpErrorBody::invalid_params("tools/call params.name is required"))?;
    let arguments = parsed
        .arguments
        .and_then(|value| (!value.is_null()).then_some(value));

    match tool_name.as_str() {
        "mega/generate" => {
            let output = super::handle_generate(state, arguments).await?;
            Ok(json!({
                "content": [
                    {
                        "type": "text",
                        "text": "Mega.nz account generation complete."
                    }
                ],
                "structuredContent": output
            }))
        }
        _ => Err(McpErrorBody::invalid_params("unknown tool")),
    }
}
