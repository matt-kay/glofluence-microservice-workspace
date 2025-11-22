use std::collections::HashMap;

use crate::{
    domain::{
        shared::{
            error::DomainError,
            event::EventMeta,
            value_object::{EventId, OcurredAt, SocialMediaProfiles},
        },
        term::value_objects::TermId,
        user::{
            User,
            events::UserDomainEvent,
            ports::{event::UserDomainEventBus, respository::UserRepository},
            value_object::{FirstName, LastName, UserId},
        },
    },
    predule::InMemoryUserEventBus,
};

pub struct UserService<R: UserRepository> {
    repo: R,
    bus: Box<dyn UserDomainEventBus>,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self {
            repo,
            bus: Box::new(InMemoryUserEventBus::default()),
        }
    }

    pub fn with_bus(mut self, bus: Box<dyn UserDomainEventBus>) -> Self {
        self.bus = bus;
        self
    }

    pub async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let user = self.repo.find_by_id(id).await?;
        Ok(user)
    }

    pub async fn create_user(
        &mut self,
        first_name: FirstName,
        last_name: LastName,
        country_term_id: TermId,
    ) -> Result<User, DomainError> {
        let id = UserId::new();

        let social_profiles = SocialMediaProfiles::new(vec![
            // SocialMediaMetadata::new(SocialMediaPlatform::Facebook, "AliceFB", "https://fb.com/alice", false, false, 100, None).unwrap(),
            // SocialMediaMetadata::new(SocialMediaPlatform::Instagram, "AliceIG", "https://instagram.com/alice", false, false, 250, None).unwrap(),
        ]);

        let demographics = HashMap::new();

        let mut user = User::new(
            id,
            first_name,
            last_name,
            country_term_id,
            social_profiles,
            demographics,
        );

        self.repo.save(&user).await?;

        let events = user.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(user)
    }

    pub async fn update_user(
        &mut self,
        user_id: UserId,
        first_name: Option<FirstName>,
        last_name: Option<LastName>,
        country_term_id: Option<TermId>,
    ) -> Result<User, DomainError> {
        let mut user = self
            .repo
            .find_by_id(&user_id)
            .await?
            .ok_or(DomainError::not_found("user"))?;

        if let Some(f) = first_name {
            user.set_first_name(f);
        }

        if let Some(l) = last_name {
            user.set_last_name(l);
        }

        if let Some(c) = country_term_id {
            user.set_country(c);
        }

        self.repo.save(&user).await?;

        let events = user.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(user)
    }

    pub async fn soft_delete_user(&mut self, user_id: UserId) -> Result<User, DomainError> {
        let mut user = self
            .repo
            .find_by_id(&user_id)
            .await?
            .ok_or(DomainError::not_found("user"))?;

        user.mark_as_deleted();

        self.repo.save(&user).await?;

        let events = user.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(user)
    }

    pub async fn restore_soft_deleted_user(
        &mut self,
        user_id: UserId,
    ) -> Result<User, DomainError> {
        let mut user = self
            .repo
            .find_by_id(&user_id)
            .await?
            .ok_or(DomainError::not_found("user"))?;

        user.restore_from_deleted();

        self.repo.save(&user).await?;

        let events = user.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(user)
    }

    pub async fn permanetly_delete_user(&mut self, user_id: UserId) -> Result<(), DomainError> {
        let user = self
            .repo
            .find_by_id(&user_id)
            .await?
            .ok_or(DomainError::not_found("user"))?;

        self.repo.delete(&user.id).await?;

        let events = vec![UserDomainEvent::UserDeleted {
            meta: EventMeta {
                event_id: EventId::new(),
                occurred_at: OcurredAt::now(),
                aggregate_id: user.id.as_str(),
                aggregate_version: user.version,
            },
            event_name: "user.deleted".to_owned(),
        }];

        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(())
    }
}
