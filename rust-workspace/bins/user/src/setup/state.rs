use corelib::predule::{
    IUserserviceInMemoryUserRepository, InMemoryAuditLogHandler, InMemoryUserEventBus,
    InMemoryUserRepository, InMemoryWelcomeEmailHandler, UserService,
};
use tokio::sync::Mutex;
pub struct AppState {
    pub user_service: Mutex<IUserserviceInMemoryUserRepository>,
}

pub fn build_state() -> AppState {
    // User
    let user_repository = InMemoryUserRepository::new();

    let mut user_event_bus = InMemoryUserEventBus::new();
    user_event_bus
        .with(Box::new(InMemoryWelcomeEmailHandler))
        .with(Box::new(InMemoryAuditLogHandler));

    let user_service = UserService::new(user_repository).with_bus(Box::new(user_event_bus));

    AppState {
        user_service: Mutex::new(user_service),
    }
}
