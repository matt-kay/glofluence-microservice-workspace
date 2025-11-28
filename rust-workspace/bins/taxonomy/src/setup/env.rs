use env_config::predule::ServerConfig;

pub struct EnvConfig {
    pub server: ServerConfig,
    // pub database: DatabaseConfig,
}

impl EnvConfig {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            server: ServerConfig::load("TAXONOMY_SUBGRAPH"),
            // database: DatabaseConfig::load(),
        }
    }
}
