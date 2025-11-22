use crate::domain::taxonomy::events::TaxonomyDomainEvent;



pub trait TaxonomyDomainEventBus: Send + Sync {
    fn publish(&self, events: &[TaxonomyDomainEvent]) -> Result<(), String>;
}

pub trait TaxonomyDomainEventHandler: Send + Sync {
    fn handle(&self, ev: &TaxonomyDomainEvent) -> Result<(), String>;
}
