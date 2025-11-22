use serde::{Deserialize, Serialize};

use crate::domain::shared::event::EventMeta;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum TaxonomyDomainEvent {
    TaxonomyCreated {
        meta: EventMeta,
        event_name: String,
        parent_id: Option<String>,
        name: String,
        visible: bool,
        description: Option<String>,
    },
    TaxonomyUpdated {
        meta: EventMeta,
        event_name: String,
        parent_id: Option<String>,
        name: Option<String>,
        visible: Option<bool>,
        description: Option<String>,
    },
    TaxonomyDeleted {
        meta: EventMeta,
        event_name: String,
    },
    TaxonomySoftDeleted {
        meta: EventMeta,
        event_name: String,
    },
    TaxonomyRestoredFromSoftDeleted {
        meta: EventMeta,
        event_name: String,
    },
}
