use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::shared::{
    error::DomainError,
    value_object::{
        EmailAddress, PhoneNumber, PhysicalAddress, SocialMediaLink, SocialPlatformName, Tag, WebsiteUrl
    },
};

/// Unique identifier for business
/// # Field
/// - `value`- raw uuid v4 value.

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BusinessId(Uuid);

impl BusinessId {
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

/// Business name
///
/// # Field
/// - `value` - raw string value
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BusinessName(String);

impl BusinessName {
    /// Creates a new `BusinessName` value object
    ///
    /// # Errors
    /// Returns `DomainError::InvalidInput` if the value is empty or too long
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        // Basic validation
        if value.trim().is_empty() {
            return Err(DomainError::validation("Business name cannot be empty"));
        }

        if value.len() > 50 {
            return Err(DomainError::validation(
                "Business name is too long (max 50 chars)",
            ));
        }

        // Optionally: allow only alphabetic characters
        if !value
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        {
            return Err(DomainError::validation(
                "Business name contains invalid characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the string value of the business name
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Business description
///
/// # Field
/// - `value` - raw string value
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BusinessDescription(String);

impl BusinessDescription {
    /// Creates a new `BusinessName` value object
    ///
    /// # Errors
    /// Returns `DomainError::InvalidInput` if the value is empty or too long
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        // Basic validation
        if value.trim().is_empty() {
            return Err(DomainError::validation(
                "Business description cannot be empty",
            ));
        }

        if value.len() > 50 {
            return Err(DomainError::validation(
                "Business description is too long (max 50 chars)",
            ));
        }

        // Optionally: allow only alphabetic characters
        if !value
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'')
        {
            return Err(DomainError::validation(
                "Business description contains invalid characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the string value of the business description
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Business hour entry
///
/// Example: ("Mon", "9am–5pm")
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessHourEntry {
    day: String,
    hours: String,
}

impl BusinessHourEntry {
    pub fn new(day: impl Into<String>, hours: impl Into<String>) -> Result<Self, DomainError> {
        let day = day.into();
        let hours = hours.into();

        if day.trim().is_empty() {
            return Err(DomainError::validation("Day cannot be empty"));
        }

        if hours.trim().is_empty() {
            return Err(DomainError::validation("Hours cannot be empty"));
        }

        Ok(Self { day, hours })
    }

    pub fn day(&self) -> &str {
        &self.day
    }
    pub fn hours(&self) -> &str {
        &self.hours
    }
}

/// A named service provided by the business
///
/// # Field
/// - `value` - service name string
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServiceName(String);

impl ServiceName {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DomainError::validation("Service name cannot be empty"));
        }

        if value.len() > 80 {
            return Err(DomainError::validation(
                "Service name too long (max 80 chars)",
            ));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Arbitrary extra feature metadata key
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Hash, Deserialize)]
#[serde(transparent)]
pub struct ExtraFeatureKey(String);

impl ExtraFeatureKey {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DomainError::validation("Feature key cannot be empty"));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Arbitrary extra feature metadata value
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExtraFeatureValue(String);

impl ExtraFeatureValue {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DomainError::validation("Feature value cannot be empty"));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Option<EmailAddress>,
    pub phone: Option<PhoneNumber>,
    pub address: Option<PhysicalAddress>,
    pub website: Option<WebsiteUrl>,
}

impl ContactInfo {
    pub fn new(
        email: Option<EmailAddress>,
        phone: Option<PhoneNumber>,
        address: Option<PhysicalAddress>,
        website: Option<WebsiteUrl>,
    ) -> Self {
        Self {
            email,
            phone,
            address,
            website,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessFeatures {
    /// Example: Mon -> "9am–5pm"
    pub hours: Option<Vec<BusinessHourEntry>>,

    /// Services provided by the business
    pub services: Vec<ServiceName>,

    /// Descriptive tags
    pub tags: Vec<Tag>,

    /// Custom metadata
    pub extra: HashMap<ExtraFeatureKey, ExtraFeatureValue>,
}

impl BusinessFeatures {
    pub fn new(
        hours: Option<Vec<BusinessHourEntry>>,
        services: Vec<ServiceName>,
        tags: Vec<Tag>,
        extra: HashMap<ExtraFeatureKey, ExtraFeatureValue>,
    ) -> Self {
        Self {
            hours,
            services,
            tags,
            extra,
        }
    }
}
