use uuid::Uuid;

use crate::domain::shared::error::DomainError;

/// Unique identifier for user
/// # Field
/// - `value`- raw uuid v4 value.

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct UserId(Uuid);

impl UserId {
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

/// First name of a user
///
/// # Field
/// - `value` - raw string value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstName(String);

impl FirstName {
    /// Creates a new `FirstName` value object
    ///
    /// # Errors
    /// Returns `DomainError::InvalidInput` if the value is empty or too long
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        // Basic validation
        if value.trim().is_empty() {
            return Err(DomainError::validation("First name cannot be empty"));
        }

        if value.len() > 50 {
            return Err(DomainError::validation(
                "First name is too long (max 50 chars)",
            ));
        }

        // Optionally: allow only alphabetic characters
        if !value
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        {
            return Err(DomainError::validation(
                "First name contains invalid characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the string value of the first name
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Last name of a user
///
/// # Field
/// - `value` - raw string value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastName(String);

impl LastName {
    /// Creates a new `LastName` value object
    ///
    /// # Errors
    /// Returns `DomainError::InvalidInput` if the value is empty, too long, or contains invalid characters
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        // Validation: not empty
        if value.trim().is_empty() {
            return Err(DomainError::validation("Last name cannot be empty"));
        }

        // Validation: max length (example: 50)
        if value.len() > 50 {
            return Err(DomainError::validation(
                "Last name is too long (max 50 chars)",
            ));
        }

        // Validation: only alphabetic, dash, or apostrophe
        if !value
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        {
            return Err(DomainError::validation(
                "Last name contains invalid characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the string value of the last name
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
