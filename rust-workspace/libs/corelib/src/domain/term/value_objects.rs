use uuid::Uuid;

use crate::domain::shared::error::DomainError;

/// Unique identifier for term
/// # Field
/// - `value`- raw uuid v4 value.

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct TermId(Uuid);

impl TermId {
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

/// Term name of a user
///
/// # Field
/// - `value` - raw string value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermName(String);

impl TermName {
    /// Creates a new `TermName` value object
    ///
    /// # Errors
    /// Returns `DomainError::InvalidInput` if the value is empty or too long
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        // Basic validation
        if value.trim().is_empty() {
            return Err(DomainError::validation("Term name cannot be empty"));
        }

        if value.len() > 50 {
            return Err(DomainError::validation(
                "Term name is too long (max 50 chars)",
            ));
        }

        // Optionally: allow only alphabetic characters
        if !value
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        {
            return Err(DomainError::validation(
                "Term name contains invalid characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the string value of the taxonomy name
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Term description of a user
///
/// # Field
/// - `value` - raw string value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermDescription(String);

impl TermDescription {
    /// Creates a new `TermDescription` value object
    ///
    /// # Errors
    /// Returns `DomainError::InvalidInput` if the value is empty or too long
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        // Basic validation
        if value.trim().is_empty() {
            return Err(DomainError::validation("Term description cannot be empty"));
        }

        if value.len() > 50 {
            return Err(DomainError::validation(
                "Term description is too long (max 50 chars)",
            ));
        }

        // Optionally: allow only alphabetic characters
        if !value
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        {
            return Err(DomainError::validation(
                "Term description contains invalid characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the string value of the taxonomy description
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
