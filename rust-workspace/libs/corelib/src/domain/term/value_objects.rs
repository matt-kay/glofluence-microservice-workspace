use uuid::Uuid;

/// Unique identifier for term
/// # Field
/// - `value`- raw uuid v4 value.

#[derive(Debug,PartialEq, Eq,Clone,Hash)]
pub struct TermId(Uuid);

impl TermId {
    pub fn new() -> Self {
        let new_uuid = Uuid::new_v4();
        Self(new_uuid)
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}
