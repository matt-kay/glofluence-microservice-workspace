mod application;
mod domain;
mod infrastructure;

pub mod predule {
    pub use crate::domain::user::User;
    pub use crate::domain::user::value_object::UserId;
    pub use crate::domain::user::value_object::FirstName;
    pub use crate::domain::user::value_object::LastName;

    pub use crate::domain::term::Term;
    pub use crate::domain::term::value_objects::TermId;

    pub use crate::domain::shared::value_object::SocialMediaMetadata;
    pub use crate::domain::shared::value_object::SocialMediaPlatform;
    pub use crate::domain::shared::value_object::Demographics;

    pub use crate::application::user::service::UserService;

    pub use crate::infrastructure::user::event_bus::in_memory_bus::InMemoryUserEventBus;
    pub use crate::infrastructure::user::event_handlers::in_memory_handlers::InMemoryAuditLogHandler;
    pub use crate::infrastructure::user::event_handlers::in_memory_handlers::InMemoryWelcomeEmailHandler;

    pub use crate::infrastructure::user::repository::in_memory_impl::InMemoryUserRepository;
    pub use crate::infrastructure::user::repository::in_memory_impl::IUserserviceInMemoryUserRepository;
}
