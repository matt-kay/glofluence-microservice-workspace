use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl ServerConfig {
    pub fn load(prefix: &str) -> Self {
        let key = format!("{}_PORT", prefix);

        Self {
            port: std::env::var(&key)
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    }
}
