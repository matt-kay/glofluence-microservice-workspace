use corelib::predule::{
    ITaxonomyserviceInMemoryTaxonomyRepository, InMemoryTaxonomyAuditLogHandler,
    InMemoryTaxonomyEventBus, InMemoryTaxonomyRepository, TaxonomyService,
};
use tokio::sync::Mutex;
pub struct AppState {
    pub taxonomy_service: Mutex<ITaxonomyserviceInMemoryTaxonomyRepository>,
}

pub fn build_state() -> AppState {
    // Taxonomy
    let taxonomy_repository = InMemoryTaxonomyRepository::new();

    let mut taxonomy_event_bus = InMemoryTaxonomyEventBus::new();
    taxonomy_event_bus.with(Box::new(InMemoryTaxonomyAuditLogHandler));

    let taxonomy_service =
        TaxonomyService::new(taxonomy_repository).with_bus(Box::new(taxonomy_event_bus));

    AppState {
        taxonomy_service: Mutex::new(taxonomy_service),
    }
}
