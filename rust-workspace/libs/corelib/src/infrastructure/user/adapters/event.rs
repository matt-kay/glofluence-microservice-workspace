use crate::domain::user::{
    events::UserDomainEvent,
    ports::event::{UserDomainEventHandler, UserDomainEventBus},
};

#[derive(Default)]
pub struct UserInMemoryEventBus {
    handlers: Vec<Box<dyn UserDomainEventHandler>>,
}

impl UserInMemoryEventBus {
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }
    pub fn with(mut self, h: Box<dyn UserDomainEventHandler>) -> Self {
        self.handlers.push(h);
        self
    }
}

impl UserDomainEventBus for UserInMemoryEventBus {
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

// Example handlers

pub struct WelcomeEmailHandler;
impl UserDomainEventHandler for WelcomeEmailHandler {
    fn handle(&self, ev: &UserDomainEvent) -> Result<(), String> {
        if let UserDomainEvent::UserCreated {
            meta: _,
            first_name,
            ..
        } = ev
        {
            println!("[mailer] Send welcome to {first_name}");
        }
        Ok(())
    }
}

pub struct AuditLogHandler;
impl UserDomainEventHandler for AuditLogHandler {
    fn handle(&self, ev: &UserDomainEvent) -> Result<(), String> {
        println!("[audit] {:?}", ev);
        Ok(())
    }
}
