use std::env;

#[derive(Debug, Clone)]
pub struct AppState {
    pub default_count: u32,
    pub max_count: u32,
    pub default_password: String,
    pub proxy_url: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        let proxy_url = env::var("MEGA_PROXY_URL")
            .ok()
            .map(|proxy| proxy.trim().to_string())
            .filter(|proxy| !proxy.is_empty());

        Self {
            default_count: 1,
            max_count: 5,
            default_password: "Mcp!Passw0rd2026".to_string(),
            proxy_url,
        }
    }
}
