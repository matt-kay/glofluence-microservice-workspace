use corelib::predule::{
    ITermserviceInMemoryTermRepository, InMemoryTermAuditLogHandler, InMemoryTermEventBus, InMemoryTermRepository, TermService
};
use tokio::sync::Mutex;
pub struct AppState {
    pub term_service: Mutex<ITermserviceInMemoryTermRepository>,
}

pub fn build_state() -> AppState {
    // Term
    let term_repository = InMemoryTermRepository::new();

    let mut term_event_bus = InMemoryTermEventBus::new();
    term_event_bus.with(Box::new(InMemoryTermAuditLogHandler));

    let term_service = TermService::new(term_repository).with_bus(Box::new(term_event_bus));

    AppState {
        term_service: Mutex::new(term_service),
    }
}
