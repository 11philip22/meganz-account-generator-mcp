use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpErrorBody {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum McpErrorCode {
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    GenerationFailed,
}

impl McpErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRequest => "INVALID_REQUEST",
            Self::MethodNotFound => "METHOD_NOT_FOUND",
            Self::InvalidParams => "INVALID_PARAMS",
            Self::GenerationFailed => "GENERATION_FAILED",
        }
    }
}

impl McpErrorBody {
    pub fn new(code: McpErrorCode, message: impl Into<String>) -> Self {
        Self {
            code: code.as_str().to_string(),
            message: message.into(),
        }
    }

    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::new(McpErrorCode::InvalidRequest, message)
    }

    pub fn method_not_found(message: impl Into<String>) -> Self {
        Self::new(McpErrorCode::MethodNotFound, message)
    }

    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self::new(McpErrorCode::InvalidParams, message)
    }

    pub fn generation_failed(message: impl Into<String>) -> Self {
        Self::new(McpErrorCode::GenerationFailed, message)
    }
}
