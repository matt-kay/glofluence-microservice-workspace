mod configs;

pub mod predule {
    pub use crate::configs::auth::AuthConfig;
    pub use crate::configs::database::DatabaseConfig;
    pub use crate::configs::server::ServerConfig;
}
