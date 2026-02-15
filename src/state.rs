#[derive(Debug, Clone)]
pub struct AppState {
    pub default_count: u32,
    pub max_count: u32,
    pub default_password: String,
    pub proxy_url: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(None)
    }
}

impl AppState {
    pub fn new(proxy_url_override: Option<String>) -> Self {
        let proxy_url = match proxy_url_override {
            Some(proxy) => normalize_proxy(proxy),
            None => None,
        };

        Self {
            default_count: 1,
            max_count: 5,
            default_password: "Mcp!Passw0rd2026".to_string(),
            proxy_url,
        }
    }
}

fn normalize_proxy(proxy: String) -> Option<String> {
    let trimmed = proxy.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
