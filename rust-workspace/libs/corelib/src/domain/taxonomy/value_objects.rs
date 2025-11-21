use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for taxonomy
/// # Field
/// - `value`- raw uuid v4 value.

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct TaxonomyId(Uuid);

impl TaxonomyId {
    pub fn new() -> Self {
        let new_uuid = Uuid::new_v4();
        Self(new_uuid)
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}
