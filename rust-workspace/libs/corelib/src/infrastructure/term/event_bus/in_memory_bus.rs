use crate::domain::term::{
    events::TermDomainEvent,
    ports::event::{TermDomainEventBus, TermDomainEventHandler},
};

#[derive(Default)]
pub struct InMemoryTermEventBus {
    handlers: Vec<Box<dyn TermDomainEventHandler>>,
}

impl InMemoryTermEventBus {
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }
  pub fn with(&mut self, h: Box<dyn TermDomainEventHandler>) -> &mut Self {
        self.handlers.push(h);
        self
    }
}

impl TermDomainEventBus for InMemoryTermEventBus {
    fn publish(&self, events: &[TermDomainEvent]) -> Result<(), String> {
        for ev in events {
            for h in &self.handlers {
                // In production, consider error routing / retries
                h.handle(ev)?;
            }
        }
        Ok(())
    }
}
