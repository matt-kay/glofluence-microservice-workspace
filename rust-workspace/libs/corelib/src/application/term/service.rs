use crate::domain::{
    shared::{
        error::DomainError,
        event::EventMeta,
        value_object::{EventId, OcurredAt},
    },
    taxonomy::value_objects::TaxonomyId,
    term::{
        Term,
        events::TermDomainEvent,
        ports::{event::TermDomainEventBus, respository::TermRepository},
        value_objects::{TermDescription, TermId, TermName},
    },
};

use crate::infrastructure::term::event_bus::in_memory_bus::InMemoryTermEventBus;

pub struct TermService<R: TermRepository> {
    repo: R,
    bus: Box<dyn TermDomainEventBus>,
}

impl<R: TermRepository> TermService<R> {
    pub fn new(repo: R) -> Self {
        Self {
            repo,
            bus: Box::new(InMemoryTermEventBus::default()),
        }
    }

    pub fn with_bus(mut self, bus: Box<dyn TermDomainEventBus>) -> Self {
        self.bus = bus;
        self
    }

    pub async fn find_by_id(&self, id: &TermId) -> Result<Option<Term>, DomainError> {
        let term = self.repo.find_by_id(id).await?;
        Ok(term)
    }

    pub async fn create_term(
        &mut self,
        taxonomy_id: TaxonomyId,
        parent_id: Option<TermId>,
        name: TermName,
        visible: bool,
        description: Option<TermDescription>,
    ) -> Result<Term, DomainError> {
        let id = TermId::new();

        let mut term = Term::new(id, taxonomy_id, parent_id, name, visible, description);

        self.repo.save(&term).await?;

        let events = term.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(term)
    }

    pub async fn update_term(
        &mut self,
        term_id: TermId,
        taxonomy_id: Option<TaxonomyId>,
        parent_id: Option<TermId>,
        name: Option<TermName>,
        visible: Option<bool>,
        description: Option<TermDescription>,
    ) -> Result<Term, DomainError> {
        let mut term = self
            .repo
            .find_by_id(&term_id)
            .await?
            .ok_or(DomainError::not_found("term"))?;

        if let Some(v) = taxonomy_id {
            term.set_taxonomy_id(v);
        }

        if let Some(v) = parent_id {
            term.set_parent_id(v);
        }

        if let Some(v) = name {
            term.set_name(v);
        }
        if let Some(v) = visible {
            term.set_visible(v);
        }
        if let Some(v) = description {
            term.set_description(v);
        }

        self.repo.save(&term).await?;

        let events = term.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(term)
    }

    pub async fn soft_delete_term(&mut self, term_id: TermId) -> Result<Term, DomainError> {
        let mut term = self
            .repo
            .find_by_id(&term_id)
            .await?
            .ok_or(DomainError::not_found("term"))?;

        term.mark_as_deleted();

        self.repo.save(&term).await?;

        let events = term.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(term)
    }

    pub async fn restore_soft_deleted_term(
        &mut self,
        term_id: TermId,
    ) -> Result<Term, DomainError> {
        let mut term = self
            .repo
            .find_by_id(&term_id)
            .await?
            .ok_or(DomainError::not_found("term"))?;

        term.restore_from_deleted();

        self.repo.save(&term).await?;

        let events = term.take_events();
        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(term)
    }

    pub async fn permanetly_delete_term(&mut self, term_id: TermId) -> Result<(), DomainError> {
        let term = self
            .repo
            .find_by_id(&term_id)
            .await?
            .ok_or(DomainError::not_found("term"))?;

        self.repo.delete(&term.id).await?;

        let events = vec![TermDomainEvent::TermDeleted {
            meta: EventMeta {
                event_id: EventId::new(),
                occurred_at: OcurredAt::now(),
                aggregate_id: term.id.as_str(),
                aggregate_version: term.version,
            },
            event_name: "term.deleted".to_owned(),
        }];

        self.bus
            .publish(&events)
            .map_err(|e| DomainError::conflict(format!("failed to publish events: {}", e)))?;
        Ok(())
    }
}
