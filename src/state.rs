#[derive(Debug, Clone)]
pub struct AppState {
    pub max_count: u32,
}

impl Default for AppState {
    fn default() -> Self {
        Self { max_count: 5 }
    }
}
