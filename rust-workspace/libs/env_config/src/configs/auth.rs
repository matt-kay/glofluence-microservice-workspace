#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
}

impl AuthConfig {
    pub fn load() -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET missing"),
        }
    }
}
