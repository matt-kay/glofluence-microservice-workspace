pub mod events;
pub mod value_objects;
pub mod ports;

use crate::domain::{
    shared::{
        event::EventMeta,
        value_object::{Deleted, EventId, OcurredAt, Timestamp},
    },
    taxonomy::value_objects::TaxonomyId,
    term::{
        events::TermDomainEvent,
        value_objects::{TermDescription, TermId, TermName},
    },
};

#[derive(Debug, Clone)]
pub struct Term {
    pub id: TermId,
    pub taxonomy_id: TaxonomyId,
    pub parent_id: Option<TermId>,

    pub name: TermName,
    pub visible: bool,
    pub description: Option<TermDescription>,

    pub timestamps: Timestamp,
    pub deleted: Deleted,

    pub version: u64,

    pending_events: Vec<TermDomainEvent>,
}

impl Term {
    pub fn new(
        id: TermId,
        taxonomy_id: TaxonomyId,
        parent_id: Option<TermId>,
        name: TermName,
        visible: bool,
        description: Option<TermDescription>,
    ) -> Self {
        let mut term = Term {
            id,
            taxonomy_id: taxonomy_id.clone(),
            parent_id: parent_id.clone(),
            name: name.clone(),
            visible,
            description: description.clone(),
            version: 0,
            timestamps: Timestamp::new(),
            deleted: Deleted::new(),
            pending_events: Vec::new(),
        };

        term.pending_events.push(TermDomainEvent::TermCreated {
            meta: EventMeta {
                event_id: EventId::new(),
                occurred_at: OcurredAt::now(),
                aggregate_id: id.as_str(),
                aggregate_version: 1,
            },
            event_name: "term.created".to_owned(),
            taxonomy_id: taxonomy_id.as_str().to_owned(),
            parent_id: parent_id.map(|v| v.as_str().to_owned()),
            name: name.as_str().to_owned(),
            visible: visible.to_owned(),
            description: description.map(|v| v.as_str().to_owned()),
        });

        term
    }
    pub fn set_taxonomy_id(&mut self, taxonomy_id: TaxonomyId) {
        self.taxonomy_id = taxonomy_id.clone();
        self.touch();
        self.pending_events.push(TermDomainEvent::TermUpdated {
            meta: self.next_meta(),
            event_name: "term.updated".to_owned(),
            taxonomy_id: Some(taxonomy_id.as_str().to_owned()),
            parent_id: None,
            name: None,
            visible: None,
            description: None,
        });
    }

    pub fn set_parent_id(&mut self, parent_id: TermId) {
        self.parent_id = Some(parent_id.clone());
        self.touch();
        self.pending_events.push(TermDomainEvent::TermUpdated {
            meta: self.next_meta(),
            event_name: "term.updated".to_owned(),
            taxonomy_id: None,
            parent_id: Some(parent_id.as_str().to_owned()),
            name: None,
            visible: None,
            description: None,
        });
    }

    pub fn set_name(&mut self, name: TermName) {
        self.name = name.clone();
        self.touch();
        self.pending_events.push(TermDomainEvent::TermUpdated {
            meta: self.next_meta(),
            event_name: "term.updated".to_owned(),
            taxonomy_id: None,
            parent_id: None,
            name: Some(name.as_str().to_owned()),
            visible: None,
            description: None,
        });
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible.clone();
        self.touch();
        self.pending_events.push(TermDomainEvent::TermUpdated {
            meta: self.next_meta(),
            event_name: "term.updated".to_owned(),
            taxonomy_id: None,
            parent_id: None,
            name: None,
            visible: Some(visible.to_owned()),
            description: None,
        });
    }

    pub fn set_description(&mut self, description: TermDescription) {
        self.description = Some(description.clone());
        self.touch();
        self.pending_events.push(TermDomainEvent::TermUpdated {
            meta: self.next_meta(),
            event_name: "term.updated".to_owned(),
            taxonomy_id: None,
            parent_id: None,
            name: None,
            visible: None,
            description: Some(description.as_str().to_owned()),
        });
    }

    pub fn mark_as_deleted(&mut self) {
        self.deleted.mark_deleted();
        self.touch();
        self.pending_events.push(TermDomainEvent::TermSoftDeleted {
            meta: self.next_meta(),
            event_name: "term.soft_deleted".to_owned(),
        });
    }

    pub fn restore_from_deleted(&mut self) {
        self.deleted.restore();
        self.touch();
        self.pending_events
            .push(TermDomainEvent::TermRestoredFromSoftDeleted {
                meta: self.next_meta(),
                event_name: "term.restored_from_soft_deleted".to_owned(),
            });
    }

    fn next_meta(&self) -> EventMeta {
        EventMeta {
            event_id: EventId::new(),
            occurred_at: OcurredAt::now(),
            aggregate_id: self.id.as_str(),
            aggregate_version: self.version + 1,
        }
    }

    pub fn take_events(&mut self) -> Vec<TermDomainEvent> {
        std::mem::take(&mut self.pending_events)
    }

    fn touch(&mut self) {
        self.timestamps.touch();
        self.version += 1;
    }
}
