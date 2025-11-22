#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn load() -> Self {
        Self {
            url: std::env::var("DATABASE_URL").expect("DATABASE_URL missing"),
        }
    }
}
