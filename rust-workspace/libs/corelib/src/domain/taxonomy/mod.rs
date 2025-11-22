pub mod events;
pub mod value_objects;
pub mod ports;

use crate::domain::{
    shared::{
        event::EventMeta,
        value_object::{Deleted, EventId, OcurredAt, Timestamp},
    },
    taxonomy::{
        events::TaxonomyDomainEvent,
        value_objects::{TaxonomyDescription, TaxonomyId, TaxonomyName},
    },
};

#[derive(Debug, Clone)]
pub struct Taxonomy {
    pub id: TaxonomyId,
    pub parent_id: Option<TaxonomyId>,

    pub name: TaxonomyName,
    pub visible: bool,
    pub description: Option<TaxonomyDescription>,

    pub timestamps: Timestamp,
    pub deleted: Deleted,

    pub version: u64,

    pending_events: Vec<TaxonomyDomainEvent>,
}

impl Taxonomy {
    pub fn new(
        id: TaxonomyId,
        parent_id: Option<TaxonomyId>,
        name: TaxonomyName,
        visible: bool,
        description: Option<TaxonomyDescription>,
    ) -> Self {
        let mut taxonomy = Taxonomy {
            id,
            parent_id: parent_id.clone(),
            name: name.clone(),
            visible,
            description: description.clone(),
            version: 0,
            timestamps: Timestamp::new(),
            deleted: Deleted::new(),
            pending_events: Vec::new(),
        };

        taxonomy
            .pending_events
            .push(TaxonomyDomainEvent::TaxonomyCreated {
                meta: EventMeta {
                    event_id: EventId::new(),
                    occurred_at: OcurredAt::now(),
                    aggregate_id: id.as_str(),
                    aggregate_version: 1,
                },
                event_name: "taxonomy.created".to_owned(),
                parent_id: parent_id.map(|v| v.as_str().to_owned()),
                name: name.as_str().to_owned(),
                visible: visible.to_owned(),
                description: description.map(|v| v.as_str().to_owned()),
            });

        taxonomy
    }

    pub fn set_parent_id(&mut self, parent_id: TaxonomyId) {
        self.parent_id = Some(parent_id.clone());
        self.touch();
        self.pending_events
            .push(TaxonomyDomainEvent::TaxonomyUpdated {
                meta: self.next_meta(),
                event_name: "taxonomy.updated".to_owned(),
                parent_id: Some(parent_id.as_str().to_owned()),
                name: None,
                visible: None,
                description: None,
            });
    }

    pub fn set_name(&mut self, name: TaxonomyName) {
        self.name = name.clone();
        self.touch();
        self.pending_events
            .push(TaxonomyDomainEvent::TaxonomyUpdated {
                meta: self.next_meta(),
                event_name: "taxonomy.updated".to_owned(),
                parent_id: None,
                name: Some(name.as_str().to_owned()),
                visible: None,
                description: None,
            });
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible.clone();
        self.touch();
        self.pending_events
            .push(TaxonomyDomainEvent::TaxonomyUpdated {
                meta: self.next_meta(),
                event_name: "taxonomy.updated".to_owned(),
                parent_id: None,
                name: None,
                visible: Some(visible.to_owned()),
                description: None,
            });
    }

    pub fn set_description(&mut self, description: TaxonomyDescription) {
        self.description = Some(description.clone());
        self.touch();
        self.pending_events
            .push(TaxonomyDomainEvent::TaxonomyUpdated {
                meta: self.next_meta(),
                event_name: "taxonomy.updated".to_owned(),
                parent_id: None,
                name: None,
                visible: None,
                description: Some(description.as_str().to_owned()),
            });
    }

    pub fn mark_as_deleted(&mut self) {
        self.deleted.mark_deleted();
        self.touch();
        self.pending_events
            .push(TaxonomyDomainEvent::TaxonomySoftDeleted {
                meta: self.next_meta(),
                event_name: "taxonomy.soft_deleted".to_owned(),
            });
    }

    pub fn restore_from_deleted(&mut self) {
        self.deleted.restore();
        self.touch();
        self.pending_events
            .push(TaxonomyDomainEvent::TaxonomyRestoredFromSoftDeleted {
                meta: self.next_meta(),
                event_name: "taxonomy.restored_from_soft_deleted".to_owned(),
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

    pub fn take_events(&mut self) -> Vec<TaxonomyDomainEvent> {
        std::mem::take(&mut self.pending_events)
    }

    fn touch(&mut self) {
        self.timestamps.touch();
        self.version += 1;
    }
}
