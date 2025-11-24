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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstName(String);

impl FirstName {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(DomainError::validation("First name cannot be empty"));
        }

        if trimmed.chars().count() > 50 {
            return Err(DomainError::validation(
                "First name is too long (max 50 chars)",
            ));
        }

        // Allow Unicode letters, dash, or apostrophe
        if !trimmed
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        {
            return Err(DomainError::validation(
                "First name contains invalid characters",
            ));
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Last name of a user
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastName(String);

impl LastName {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(DomainError::validation("Last name cannot be empty"));
        }

        if trimmed.chars().count() > 50 {
            return Err(DomainError::validation(
                "Last name is too long (max 50 chars)",
            ));
        }

        if !trimmed
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        {
            return Err(DomainError::validation(
                "Last name contains invalid characters",
            ));
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
