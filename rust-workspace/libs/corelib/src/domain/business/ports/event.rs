use crate::domain::business::events::BusinessDomainEvent;



pub trait BusinessDomainEventBus: Send + Sync {
    fn publish(&self, events: &[BusinessDomainEvent]) -> Result<(), String>;
}

pub trait BusinessDomainEventHandler: Send + Sync {
    fn handle(&self, ev: &BusinessDomainEvent) -> Result<(), String>;
}
