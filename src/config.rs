#[derive(Clone)]
pub struct Config {
    pub version: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            version: std::env::var("VERSION").unwrap_or_else(|_| "N/A".to_string()),
        }
    }
}
