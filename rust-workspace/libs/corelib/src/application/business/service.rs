use crate::domain::{
    business::{
        Business,
        events::BusinessDomainEvent,
        ports::{event::BusinessDomainEventBus, respository::BusinessRepository},
        value_objects::{
            BusinessDescription, BusinessFeatures, BusinessId, BusinessName, ContactInfo,
        },
    },
    shared::{
        error::DomainError,
        event::EventMeta,
        value_object::{EventId, OcurredAt, SocialMedia},
    },
};

use crate::infrastructure::business::event_bus::in_memory_bus::InMemoryBusinessEventBus;

pub struct BusinessService<R: BusinessRepository> {
    repo: R,
    bus: Box<dyn BusinessDomainEventBus>,
}

impl<R: BusinessRepository> BusinessService<R> {
    pub fn new(repo: R) -> Self {
        Self {
            repo,
            bus: Box::new(InMemoryBusinessEventBus::default()),
        }
    }

    pub fn with_bus(mut self, bus: Box<dyn BusinessDomainEventBus>) -> Self {
        self.bus = bus;
        self
    }

    pub async fn find_by_id(&self, id: &BusinessId) -> Result<Option<Business>, DomainError> {
        let business = self.repo.find_by_id(id).await?;
        Ok(business)
    }

    pub async fn create_business(
        &mut self,
        name: BusinessName,
        description: Option<BusinessDescription>,
        contact_info: ContactInfo,
        social_media: SocialMedia,
        features: BusinessFeatures,
    ) -> Result<Business, DomainError> {
        let id = BusinessId::new();

        let mut business =
            Business::new(id, name, description, contact_info, social_media, features);

        self.repo.save(&business).await?;

        let events = business.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(business)
    }

    pub async fn update_business(
        &mut self,
        business_id: BusinessId,
        name: Option<BusinessName>,
        description: Option<BusinessDescription>,
        contact_info: Option<ContactInfo>,
        social_media: Option<SocialMedia>,
        features: Option<BusinessFeatures>,
    ) -> Result<Business, DomainError> {
        let mut business = self
            .repo
            .find_by_id(&business_id)
            .await?
            .ok_or(DomainError::not_found("business"))?;

        if let Some(v) = name {
            business.set_name(v);
        }

        if let Some(v) = description {
            business.set_description(v);
        }

        if let Some(v) = contact_info {
            business.set_contact_info(v);
        }
        if let Some(v) = social_media {
            business.set_social_media(v);
        }
        if let Some(v) = features {
            business.set_features(v);
        }

        self.repo.save(&business).await?;

        let events = business.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(business)
    }

    pub async fn soft_delete_business(
        &mut self,
        business_id: BusinessId,
    ) -> Result<Business, DomainError> {
        let mut business = self
            .repo
            .find_by_id(&business_id)
            .await?
            .ok_or(DomainError::not_found("business"))?;

        business.mark_as_deleted();

        self.repo.save(&business).await?;

        let events = business.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(business)
    }

    pub async fn restore_soft_deleted_business(
        &mut self,
        business_id: BusinessId,
    ) -> Result<Business, DomainError> {
        let mut business = self
            .repo
            .find_by_id(&business_id)
            .await?
            .ok_or(DomainError::not_found("business"))?;

        business.restore_from_deleted();

        self.repo.save(&business).await?;

        let events = business.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(business)
    }

    pub async fn permanetly_delete_business(
        &mut self,
        business_id: BusinessId,
    ) -> Result<(), DomainError> {
        let business = self
            .repo
            .find_by_id(&business_id)
            .await?
            .ok_or(DomainError::not_found("business"))?;

        self.repo.delete(&business.id).await?;

        let events = vec![BusinessDomainEvent::BusinessDeleted {
            meta: EventMeta {
                event_id: EventId::new(),
                occurred_at: OcurredAt::now(),
                aggregate_id: business.id.as_str(),
                aggregate_version: business.version,
            },
            event_name: "business.deleted".to_owned(),
        }];

        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(())
    }
}
