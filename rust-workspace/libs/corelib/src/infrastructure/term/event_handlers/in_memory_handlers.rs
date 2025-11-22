use crate::domain::term::{events::TermDomainEvent, ports::event::TermDomainEventHandler};

pub struct InMemoryTermAuditLogHandler;
impl TermDomainEventHandler for InMemoryTermAuditLogHandler {
    fn handle(&self, ev: &TermDomainEvent) -> Result<(), String> {
        dbg!(format!("[audit] {:?}", ev));
        Ok(())
    }
}
