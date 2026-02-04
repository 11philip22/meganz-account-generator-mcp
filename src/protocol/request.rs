use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpRequest {
    #[serde(default)]
    pub jsonrpc: Option<String>,
    pub id: Value,
    pub method: String,
    pub params: Option<Value>,
}
