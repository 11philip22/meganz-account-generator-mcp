use serde_json::{json, Value};

use crate::protocol::McpErrorBody;
use crate::state::AppState;

pub async fn handle_tool_call(
    state: &AppState,
    params: Option<Value>,
) -> Result<Value, McpErrorBody> {
    let params =
        params.ok_or_else(|| McpErrorBody::invalid_params("tools/call requires params object"))?;
    let params_obj = params
        .as_object()
        .ok_or_else(|| McpErrorBody::invalid_params("tools/call params must be an object"))?;

    let tool_name = params_obj
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| McpErrorBody::invalid_params("tools/call params.name is required"))?;

    let arguments = match params_obj.get("arguments") {
        Some(Value::Null) | None => None,
        Some(value) => Some(value.clone()),
    };

    match tool_name {
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
