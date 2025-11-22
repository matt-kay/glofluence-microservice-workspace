use crate::domain::user::{events::UserDomainEvent, ports::event::UserDomainEventHandler};

pub struct InMemoryWelcomeEmailHandler;
impl UserDomainEventHandler for InMemoryWelcomeEmailHandler {
    fn handle(&self, ev: &UserDomainEvent) -> Result<(), String> {
        if let UserDomainEvent::UserCreated {
            meta: _,
            first_name,
            ..
        } = ev
        {
             dbg!(format!("[mailer] Send welcome to {}",first_name));
        }
        Ok(())
    }
}

pub struct InMemoryAuditLogHandler;
impl UserDomainEventHandler for InMemoryAuditLogHandler {
    fn handle(&self, ev: &UserDomainEvent) -> Result<(), String> {
         dbg!(format!("[audit] {:?}", ev));
        Ok(())
    }
}
