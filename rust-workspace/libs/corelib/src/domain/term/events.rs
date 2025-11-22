use serde::{Deserialize, Serialize};

use crate::domain::shared::event::EventMeta;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum TermDomainEvent {
    TermCreated {
        meta: EventMeta,
        event_name: String,
        taxonomy_id: String,
        parent_id: Option<String>,
        name: String,
        visible: bool,
        description: Option<String>,
    },
    TermUpdated {
        meta: EventMeta,
        event_name: String,
        taxonomy_id: Option<String>,
        parent_id: Option<String>,
        name: Option<String>,
        visible: Option<bool>,
        description: Option<String>,
    },
    TermDeleted {
        meta: EventMeta,
        event_name: String,
    },
    TermSoftDeleted {
        meta: EventMeta,
        event_name: String,
    },
    TermRestoredFromSoftDeleted {
        meta: EventMeta,
        event_name: String,
    },
}
