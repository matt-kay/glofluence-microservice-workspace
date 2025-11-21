use crate::domain::user::events::UserDomainEvent;

pub trait UserDomainEventBus: Send + Sync {
    fn publish(&self, events: &[UserDomainEvent]) -> Result<(), String>;
}

pub trait UserDomainEventHandler: Send + Sync {
    fn handle(&self, ev: &UserDomainEvent) -> Result<(), String>;
}
