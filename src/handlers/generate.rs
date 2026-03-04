use meganz_account_generator::AccountGenerator;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const DEFAULT_COUNT: u32 = 1;
pub const MAX_COUNT: u32 = 5;
pub const DEFAULT_PASSWORD: &str = "Mcp!Passw0rd2026";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Arguments for mega/generate tool")]
pub struct GenerateArgs {
    #[schemars(description = "Number of accounts to generate")]
    pub count: Option<u32>,
    #[schemars(description = "Password for generated accounts")]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateParams {
    pub count: Option<u32>,
    pub password: Option<String>,
}

pub async fn handle_generate(
    params: Option<Value>,
    proxy_url: Option<String>,
) -> Result<Value, anyhow::Error> {
    let parsed_params = parse_params(params)?;
    let count = parsed_params.count.unwrap_or(DEFAULT_COUNT);
    let password = parsed_params
        .password
        .unwrap_or_else(|| DEFAULT_PASSWORD.to_string());

    if !(1..=MAX_COUNT).contains(&count) {
        anyhow::bail!("count must be between 1 and {}", MAX_COUNT);
    }
    if password.trim().is_empty() {
        anyhow::bail!("password cannot be empty");
    }

    let proxy = proxy_url
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string());
    let mut builder = AccountGenerator::builder();
    if let Some(url) = &proxy {
        builder = builder.proxy(url);
    }
    let generator = builder
        .build()
        .await
        .map_err(|_| anyhow::anyhow!("failed to initialize MEGA account generator"))?;

    let mut accounts = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let account = generator
            .generate(&password)
            .await
            .map_err(|err| anyhow::anyhow!("account generation failed: {err}"))?;

        accounts.push(json!({
            "email": account.email,
            "password": account.password,
            "name": account.name,
        }));
    }

    Ok(json!({ "accounts": accounts }))
}

fn parse_params(params: Option<Value>) -> Result<GenerateParams, anyhow::Error> {
    match params {
        None => Ok(GenerateParams {
            count: None,
            password: None,
        }),
        Some(value) => {
            serde_json::from_value(value).map_err(|_| anyhow::anyhow!("params must be an object"))
        }
    }
}
