use crate::domain::business::{events::BusinessDomainEvent, ports::event::BusinessDomainEventHandler};

pub struct InMemoryBusinessAuditLogHandler;
impl BusinessDomainEventHandler for InMemoryBusinessAuditLogHandler {
    fn handle(&self, ev: &BusinessDomainEvent) -> Result<(), String> {
        dbg!(format!("[audit] {:?}", ev));
        Ok(())
    }
}
