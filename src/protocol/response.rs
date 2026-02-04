use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::protocol::error::McpErrorBody;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpResponse {
    pub id: String,
    pub result: Option<Value>,
    pub error: Option<McpErrorBody>,
}

impl McpResponse {
    pub fn ok(id: impl Into<String>, result_json: Value) -> Self {
        Self {
            id: id.into(),
            result: Some(result_json),
            error: None,
        }
    }

    pub fn err(id: impl Into<String>, error: McpErrorBody) -> Self {
        Self {
            id: id.into(),
            result: None,
            error: Some(error),
        }
    }
}
