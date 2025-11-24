use corelib::predule::{
    IBusinessserviceInMemoryBusinessRepository, InMemoryBusinessAuditLogHandler, InMemoryBusinessEventBus, InMemoryBusinessRepository, BusinessService
};
use tokio::sync::Mutex;
pub struct AppState {
    pub business_service: Mutex<IBusinessserviceInMemoryBusinessRepository>,
}

pub fn build_state() -> AppState {
    // Business
    let business_repository = InMemoryBusinessRepository::new();

    let mut business_event_bus = InMemoryBusinessEventBus::new();
    business_event_bus.with(Box::new(InMemoryBusinessAuditLogHandler));

    let business_service = BusinessService::new(business_repository).with_bus(Box::new(business_event_bus));

    AppState {
        business_service: Mutex::new(business_service),
    }
}
