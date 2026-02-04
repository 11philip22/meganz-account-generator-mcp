use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpErrorBody {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum McpErrorCode {
    INVALID_REQUEST,
    METHOD_NOT_FOUND,
    INVALID_PARAMS,
    INTERNAL_ERROR,
    GENERATION_FAILED,
}

impl McpErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::INVALID_REQUEST => "INVALID_REQUEST",
            Self::METHOD_NOT_FOUND => "METHOD_NOT_FOUND",
            Self::INVALID_PARAMS => "INVALID_PARAMS",
            Self::INTERNAL_ERROR => "INTERNAL_ERROR",
            Self::GENERATION_FAILED => "GENERATION_FAILED",
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
        Self::new(McpErrorCode::INVALID_REQUEST, message)
    }

    pub fn method_not_found(message: impl Into<String>) -> Self {
        Self::new(McpErrorCode::METHOD_NOT_FOUND, message)
    }

    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self::new(McpErrorCode::INVALID_PARAMS, message)
    }

    pub fn generation_failed(message: impl Into<String>) -> Self {
        Self::new(McpErrorCode::GENERATION_FAILED, message)
    }
}
