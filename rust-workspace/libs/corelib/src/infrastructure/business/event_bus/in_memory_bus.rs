use crate::domain::business::{
    events::BusinessDomainEvent,
    ports::event::{BusinessDomainEventBus, BusinessDomainEventHandler},
};

#[derive(Default)]
pub struct InMemoryBusinessEventBus {
    handlers: Vec<Box<dyn BusinessDomainEventHandler>>,
}

impl InMemoryBusinessEventBus {
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }
    pub fn with(&mut self, h: Box<dyn BusinessDomainEventHandler>) -> &mut Self {
        self.handlers.push(h);
        self
    }
}

impl BusinessDomainEventBus for InMemoryBusinessEventBus {
    fn publish(&self, events: &[BusinessDomainEvent]) -> Result<(), String> {
        for ev in events {
            for h in &self.handlers {
                // In production, consider error routing / retries
                h.handle(ev)?;
            }
        }
        Ok(())
    }
}
