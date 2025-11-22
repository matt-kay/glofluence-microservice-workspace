use crate::domain::user::{
    events::UserDomainEvent,
    ports::event::{UserDomainEventBus, UserDomainEventHandler},
};

#[derive(Default)]
pub struct InMemoryUserEventBus {
    handlers: Vec<Box<dyn UserDomainEventHandler>>,
}

impl InMemoryUserEventBus {
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }
  pub fn with(&mut self, h: Box<dyn UserDomainEventHandler>) -> &mut Self {
        self.handlers.push(h);
        self
    }
}

impl UserDomainEventBus for InMemoryUserEventBus {
    fn publish(&self, events: &[UserDomainEvent]) -> Result<(), String> {
        for ev in events {
            for h in &self.handlers {
                // In production, consider error routing / retries
                h.handle(ev)?;
            }
        }
        Ok(())
    }
}
