use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpRequest {
    pub id: String,
    pub method: String,
    pub params: Option<Value>,
}
