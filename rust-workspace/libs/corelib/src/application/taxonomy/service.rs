use crate::domain::{
    shared::{
        error::DomainError,
        event::EventMeta,
        value_object::{EventId, OcurredAt},
    },
    taxonomy::{
        Taxonomy,
        events::TaxonomyDomainEvent,
        ports::{event::TaxonomyDomainEventBus, respository::TaxonomyRepository},
        value_objects::{TaxonomyDescription, TaxonomyId, TaxonomyName},
    },
};

use crate::infrastructure::taxonomy::event_bus::in_memory_bus::InMemoryTaxonomyEventBus;

pub struct TaxonomyService<R: TaxonomyRepository> {
    repo: R,
    bus: Box<dyn TaxonomyDomainEventBus>,
}

impl<R: TaxonomyRepository> TaxonomyService<R> {
    pub fn new(repo: R) -> Self {
        Self {
            repo,
            bus: Box::new(InMemoryTaxonomyEventBus::default()),
        }
    }

    pub fn with_bus(mut self, bus: Box<dyn TaxonomyDomainEventBus>) -> Self {
        self.bus = bus;
        self
    }

    pub async fn find_by_id(&self, id: &TaxonomyId) -> Result<Option<Taxonomy>, DomainError> {
        let taxonomy = self.repo.find_by_id(id).await?;
        Ok(taxonomy)
    }

    pub async fn create_taxonomy(
        &mut self,
        parent_id: Option<TaxonomyId>,
        name: TaxonomyName,
        visible: bool,
        description: Option<TaxonomyDescription>,
    ) -> Result<Taxonomy, DomainError> {
        let id = TaxonomyId::new();

        let mut taxonomy = Taxonomy::new(id, parent_id, name, visible, description);

        self.repo.save(&taxonomy).await?;

        let events = taxonomy.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(taxonomy)
    }

    pub async fn update_taxonomy(
        &mut self,
        taxonomy_id: TaxonomyId,
        parent_id: Option<TaxonomyId>,
        name: Option<TaxonomyName>,
        visible: Option<bool>,
        description: Option<TaxonomyDescription>,
    ) -> Result<Taxonomy, DomainError> {
        let mut taxonomy = self
            .repo
            .find_by_id(&taxonomy_id)
            .await?
            .ok_or(DomainError::not_found("taxonomy"))?;

        if let Some(v) = parent_id {
            taxonomy.set_parent_id(v);
        }

        if let Some(v) = name {
            taxonomy.set_name(v);
        }
        if let Some(v) = visible {
            taxonomy.set_visible(v);
        }
        if let Some(v) = description {
            taxonomy.set_description(v);
        }

        self.repo.save(&taxonomy).await?;

        let events = taxonomy.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(taxonomy)
    }

    pub async fn soft_delete_taxonomy(
        &mut self,
        taxonomy_id: TaxonomyId,
    ) -> Result<Taxonomy, DomainError> {
        let mut taxonomy = self
            .repo
            .find_by_id(&taxonomy_id)
            .await?
            .ok_or(DomainError::not_found("taxonomy"))?;

        taxonomy.mark_as_deleted();

        self.repo.save(&taxonomy).await?;

        let events = taxonomy.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(taxonomy)
    }

    pub async fn restore_soft_deleted_taxonomy(
        &mut self,
        taxonomy_id: TaxonomyId,
    ) -> Result<Taxonomy, DomainError> {
        let mut taxonomy = self
            .repo
            .find_by_id(&taxonomy_id)
            .await?
            .ok_or(DomainError::not_found("taxonomy"))?;

        taxonomy.restore_from_deleted();

        self.repo.save(&taxonomy).await?;

        let events = taxonomy.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(taxonomy)
    }

    pub async fn permanetly_delete_taxonomy(
        &mut self,
        taxonomy_id: TaxonomyId,
    ) -> Result<(), DomainError> {
        let taxonomy = self
            .repo
            .find_by_id(&taxonomy_id)
            .await?
            .ok_or(DomainError::not_found("taxonomy"))?;

        self.repo.delete(&taxonomy.id).await?;

        let events = vec![TaxonomyDomainEvent::TaxonomyDeleted {
            meta: EventMeta {
                event_id: EventId::new(),
                occurred_at: OcurredAt::now(),
                aggregate_id: taxonomy.id.as_str(),
                aggregate_version: taxonomy.version,
            },
            event_name: "taxonomy.deleted".to_owned(),
        }];

        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(())
    }
}
