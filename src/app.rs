use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
};

use crate::handlers::{GenerateArgs, handle_generate};

#[derive(Clone)]
struct MegaServer {
    proxy_url: Option<String>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl MegaServer {
    fn new(proxy_url: Option<String>) -> Self {
        Self {
            proxy_url: proxy_url
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string()),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        name = "mega/generate",
        description = "Generate Mega.nz accounts using temporary email addresses"
    )]
    async fn generate(
        &self,
        Parameters(args): Parameters<GenerateArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let params = serde_json::to_value(&args).ok();
        match handle_generate(params, self.proxy_url.clone()).await {
            Ok(value) => {
                let count = value["accounts"]
                    .as_array()
                    .map(|a| a.len())
                    .unwrap_or(0);
                let summary = format!("Generated {} account(s).", count);
                let mut result = CallToolResult::success(vec![Content::text(summary)]);
                result.structured_content = Some(value);
                Ok(result)
            }
            Err(e) => Err(ErrorData::invalid_params(e.to_string(), None)),
        }
    }
}

#[tool_handler]
impl ServerHandler for MegaServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("meganz-account-generator-mcp", "0.2.1"))
            .with_protocol_version(ProtocolVersion::V_2024_11_05)
    }
}

pub async fn run(proxy_url: Option<String>) -> anyhow::Result<()> {
    let service = MegaServer::new(proxy_url)
        .serve(rmcp::transport::stdio())
        .await?;
    service.waiting().await?;
    Ok(())
}
