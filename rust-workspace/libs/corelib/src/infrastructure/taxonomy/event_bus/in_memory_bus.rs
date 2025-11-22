use crate::domain::taxonomy::{
    events::TaxonomyDomainEvent,
    ports::event::{TaxonomyDomainEventBus, TaxonomyDomainEventHandler},
};

#[derive(Default)]
pub struct InMemoryTaxonomyEventBus {
    handlers: Vec<Box<dyn TaxonomyDomainEventHandler>>,
}

impl InMemoryTaxonomyEventBus {
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }
  pub fn with(&mut self, h: Box<dyn TaxonomyDomainEventHandler>) -> &mut Self {
        self.handlers.push(h);
        self
    }
}

impl TaxonomyDomainEventBus for InMemoryTaxonomyEventBus {
    fn publish(&self, events: &[TaxonomyDomainEvent]) -> Result<(), String> {
        for ev in events {
            for h in &self.handlers {
                // In production, consider error routing / retries
                h.handle(ev)?;
            }
        }
        Ok(())
    }
}
