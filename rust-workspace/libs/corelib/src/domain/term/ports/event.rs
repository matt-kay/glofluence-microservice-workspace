use crate::domain::term::events::TermDomainEvent;



pub trait TermDomainEventBus: Send + Sync {
    fn publish(&self, events: &[TermDomainEvent]) -> Result<(), String>;
}

pub trait TermDomainEventHandler: Send + Sync {
    fn handle(&self, ev: &TermDomainEvent) -> Result<(), String>;
}
