use meganz_account_generator::AccountGenerator;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::protocol::McpErrorBody;
use crate::state::AppState;

const DEFAULT_COUNT: u32 = 1;
const DEFAULT_PASSWORD: &str = "Mcp!Passw0rd2026";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateParams {
    pub count: Option<u32>,
}

pub async fn handle_generate(
    state: &AppState,
    params: Option<Value>,
) -> Result<Value, McpErrorBody> {
    let parsed_params = parse_params(params)?;
    let count = parsed_params.count.unwrap_or(DEFAULT_COUNT);

    if !(1..=state.max_count).contains(&count) {
        return Err(McpErrorBody::invalid_params(format!(
            "count must be between 1 and {}",
            state.max_count
        )));
    }

    let generator = AccountGenerator::new(None).await.map_err(|_| {
        McpErrorBody::generation_failed("failed to initialize MEGA account generator")
    })?;

    let mut accounts = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let account = generator
            .generate(DEFAULT_PASSWORD, None)
            .await
            .map_err(|err| {
                McpErrorBody::generation_failed(format!("account generation failed: {err}"))
            })?;

        accounts.push(json!({
            "email": account.email,
            "password": account.password,
            "name": account.name,
        }));
    }

    Ok(json!({ "accounts": accounts }))
}

fn parse_params(params: Option<Value>) -> Result<GenerateParams, McpErrorBody> {
    match params {
        None => Ok(GenerateParams { count: None }),
        Some(value) => serde_json::from_value(value)
            .map_err(|_| McpErrorBody::invalid_params("params must be an object")),
    }
}
