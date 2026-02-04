#[derive(Debug, Clone)]
pub struct AppState {
    pub max_count: u32,
    pub default_password: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            max_count: 5,
            default_password: "Mcp!Passw0rd2026".to_string(),
        }
    }
}
