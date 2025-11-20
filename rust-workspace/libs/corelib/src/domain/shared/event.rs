use serde::{Deserialize, Serialize};

use crate::domain::shared::value_object::{EventId, OcurredAt};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct EventMeta {
    pub event_id: EventId,
    pub occurred_at: OcurredAt,
    pub aggregate_id: String,
    pub aggregate_version: u64,
}
