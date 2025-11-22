mod application;
mod domain;
mod infrastructure;

pub mod predule {
    pub use crate::domain::user::User;

    pub use crate::application::user::service::UserService;

    
    pub use crate::infrastructure::user::adapters::event::AuditLogHandler;
    pub use crate::infrastructure::user::adapters::event::UserInMemoryEventBus;
    pub use crate::infrastructure::user::adapters::event::WelcomeEmailHandler;
}
