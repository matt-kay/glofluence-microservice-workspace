use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::shared::error::DomainError;

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

/// Taxonomy name of a user
///
/// # Field
/// - `value` - raw string value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaxonomyName(String);

impl TaxonomyName {
    /// Creates a new `TaxonomyName` value object
    ///
    /// # Errors
    /// Returns `DomainError::InvalidInput` if the value is empty or too long
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        // Basic validation
        if value.trim().is_empty() {
            return Err(DomainError::validation("Taxonomy name cannot be empty"));
        }

        if value.len() > 50 {
            return Err(DomainError::validation(
                "Taxonomy name is too long (max 50 chars)",
            ));
        }

        // Optionally: allow only alphabetic characters
        if !value
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        {
            return Err(DomainError::validation(
                "Taxonomy name contains invalid characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the string value of the taxonomy name
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Taxonomy description of a user
///
/// # Field
/// - `value` - raw string value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaxonomyDescription(String);

impl TaxonomyDescription {
    /// Creates a new `TaxonomyDescription` value object
    ///
    /// # Errors
    /// Returns `DomainError::InvalidInput` if the value is empty or too long
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        // Basic validation
        if value.trim().is_empty() {
            return Err(DomainError::validation(
                "Taxonomy description cannot be empty",
            ));
        }

        if value.len() > 50 {
            return Err(DomainError::validation(
                "Taxonomy description is too long (max 50 chars)",
            ));
        }

        // Optionally: allow only alphabetic characters
        if !value
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        {
            return Err(DomainError::validation(
                "Taxonomy description contains invalid characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the string value of the taxonomy description
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
