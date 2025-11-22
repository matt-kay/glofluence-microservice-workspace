use crate::domain::taxonomy::{events::TaxonomyDomainEvent, ports::event::TaxonomyDomainEventHandler};

pub struct InMemoryTaxonomyAuditLogHandler;
impl TaxonomyDomainEventHandler for InMemoryTaxonomyAuditLogHandler {
    fn handle(&self, ev: &TaxonomyDomainEvent) -> Result<(), String> {
        dbg!(format!("[audit] {:?}", ev));
        Ok(())
    }
}
