use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::domain::{shared::error::DomainError, taxonomy::value_objects::TaxonomyId};
use std::collections::HashMap;

/// Unique identifier for event
/// # Field
/// - `value`- raw uuid v4 value.

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct EventId(Uuid);

impl EventId {
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

/// OcurredAt value object for tracking ocurred time
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OcurredAt(DateTime<Utc>);

impl OcurredAt {
    /// Creates a new `OcurredAt` with the current UTC time
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Creates a `OcurredAt` from a given `DateTime<Utc>`
    pub fn from_utc(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Returns the internal `DateTime<Utc>`
    pub fn as_utc(&self) -> &DateTime<Utc> {
        &self.0
    }

    /// Returns the datetime in a **human-friendly format**
    /// Example: "Nov 20, 2025 10:45 AM"
    pub fn to_human_string(&self) -> String {
        self.0
            .with_timezone(&Local)
            .format("%b %d, %Y %I:%M %p")
            .to_string()
    }

    /// Returns just the date: "2025-11-20"
    pub fn to_date_string(&self) -> String {
        self.0.format("%Y-%m-%d").to_string()
    }

    /// Returns just the time: "10:45 AM"
    pub fn to_time_string(&self) -> String {
        self.0.with_timezone(&Local).format("%I:%M %p").to_string()
    }
}

/// Optional: implement Display for convenience
impl fmt::Display for OcurredAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_human_string())
    }
}

/// Timestamp value object for entities with creation and optional update times
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamp {
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl Timestamp {
    /// Creates a new Timestamp with `created_at = now` and `updated_at = None`
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: None,
        }
    }

    /// Creates a Timestamp from given DateTimes
    pub fn from_times(created: DateTime<Utc>, updated: Option<DateTime<Utc>>) -> Self {
        Self {
            created_at: created,
            updated_at: updated,
        }
    }

    /// Returns the created_at datetime in UTC
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Returns the updated_at datetime in UTC (or None)
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.updated_at.as_ref()
    }

    /// Marks the entity as updated now
    pub fn touch(&mut self) {
        self.updated_at = Some(Utc::now());
    }

    // ---------------- Human-friendly helpers ----------------

    /// Human-friendly created_at
    pub fn created_human(&self) -> String {
        self.created_at
            .with_timezone(&Local)
            .format("%b %d, %Y %I:%M %p")
            .to_string()
    }

    /// Human-friendly updated_at
    pub fn updated_human(&self) -> String {
        match &self.updated_at {
            Some(dt) => dt
                .with_timezone(&Local)
                .format("%b %d, %Y %I:%M %p")
                .to_string(),
            None => "Never".to_string(),
        }
    }

    /// Human-friendly created date only
    pub fn created_date(&self) -> String {
        self.created_at.format("%Y-%m-%d").to_string()
    }

    /// Human-friendly updated date only
    pub fn updated_date(&self) -> String {
        match &self.updated_at {
            Some(dt) => dt.format("%Y-%m-%d").to_string(),
            None => "Never".to_string(),
        }
    }

    /// Human-friendly created time only
    pub fn created_time(&self) -> String {
        self.created_at
            .with_timezone(&Local)
            .format("%I:%M %p")
            .to_string()
    }

    /// Human-friendly updated time only
    pub fn updated_time(&self) -> String {
        match &self.updated_at {
            Some(dt) => dt.with_timezone(&Local).format("%I:%M %p").to_string(),
            None => "Never".to_string(),
        }
    }
}

/// Implement Display for logging / debugging convenience
impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Created: {}, Updated: {}",
            self.created_human(),
            self.updated_human()
        )
    }
}

/// Soft delete info
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deleted {
    deleted: bool,
    deleted_at: Option<DateTime<Utc>>,
}

impl Deleted {
    /// Create a new "not deleted" instance
    pub fn new() -> Self {
        Self {
            deleted: false,
            deleted_at: None,
        }
    }

    /// Mark as deleted now
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.deleted_at = Some(Utc::now());
    }

    /// Restore from deleted state
    pub fn restore(&mut self) {
        self.deleted = false;
        self.deleted_at = None;
    }

    /// Check if entity is deleted
    pub fn is_deleted(&self) -> bool {
        self.deleted
    }

    /// Get deletion timestamp, if any
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.deleted_at.as_ref()
    }

    /// Human-friendly deleted status
    pub fn status(&self) -> String {
        if self.deleted {
            match &self.deleted_at {
                Some(dt) => format!(
                    "Deleted at {}",
                    dt.with_timezone(&chrono::Local)
                        .format("%b %d, %Y %I:%M %p")
                ),
                None => "Deleted".to_string(),
            }
        } else {
            "Active".to_string()
        }
    }
}

impl fmt::Display for Deleted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.status())
    }
}

pub type Demographics = HashMap<TaxonomyId, Vec<String>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocialMediaPlatform {
    Facebook,
    Instagram,
    TikTok,
    X,
    Youtube,
    LinkedIn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SocialMediaMetadata {
    platform: SocialMediaPlatform,
    profile_name: String,
    profile_link: String,
    mark_for_verification: bool,
    is_verified: bool,
    follower_count: u64,
    demographics: Option<Demographics>,
}

impl SocialMediaMetadata {
    /// Creates a new SocialMediaMetadata value object
    ///
    /// # Errors
    /// - profile_name cannot be empty if is_verified = true
    /// - profile_link must be a valid URL if is_verified = true
    pub fn new(
        platform: SocialMediaPlatform,
        profile_name: impl Into<String>,
        profile_link: impl Into<String>,
        mark_for_verification: bool,
        is_verified: bool,
        follower_count: u64,
        demographics: Option<Demographics>,
    ) -> Result<Self, DomainError> {
        let profile_name = profile_name.into();
        let profile_link = profile_link.into();

        if is_verified && profile_name.trim().is_empty() {
            return Err(DomainError::validation(format!(
                "{:?} profile name cannot be empty when verified",
                platform
            )));
        }

        if is_verified && profile_link.trim().is_empty() {
            return Err(DomainError::validation(format!(
                "{:?} profile link cannot be empty when verified",
                platform
            )));
        }

        Ok(Self {
            platform,
            profile_name,
            profile_link,
            mark_for_verification,
            is_verified,
            follower_count,
            demographics,
        })
    }

    /// Getters
    pub fn platform(&self) -> SocialMediaPlatform {
        self.platform
    }

    pub fn profile_name(&self) -> &str {
        &self.profile_name
    }

    pub fn profile_link(&self) -> &str {
        &self.profile_link
    }

    pub fn mark_for_verification(&self) -> bool {
        self.mark_for_verification
    }

    pub fn is_verified(&self) -> bool {
        self.is_verified
    }

    pub fn follower_count(&self) -> u64 {
        self.follower_count
    }

    pub fn demographics(&self) -> Option<&Demographics> {
        self.demographics.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SocialMediaProfiles {
    profiles: Vec<SocialMediaMetadata>,
}

impl SocialMediaProfiles {
    /// Creates a new collection
    pub fn new(profiles: Vec<SocialMediaMetadata>) -> Self {
        Self { profiles }
    }

    /// Returns all profiles
    pub fn all(&self) -> &Vec<SocialMediaMetadata> {
        &self.profiles
    }

    /// Find profile by platform
    pub fn get_by_platform(&self, platform: SocialMediaPlatform) -> Option<&SocialMediaMetadata> {
        self.profiles.iter().find(|p| p.platform() == platform)
    }

    /// Get all verified profiles
    pub fn verified(&self) -> Vec<&SocialMediaMetadata> {
        self.profiles.iter().filter(|p| p.is_verified()).collect()
    }

    /// Get total follower count across all profiles
    pub fn total_followers(&self) -> u64 {
        self.profiles.iter().map(|p| p.follower_count()).sum()
    }

    /// Add or update a profile in the collection
    pub fn upsert(&mut self, profile: SocialMediaMetadata) -> Result<(), DomainError> {
        if let Some(pos) = self
            .profiles
            .iter()
            .position(|p| p.platform() == profile.platform())
        {
            self.profiles[pos] = profile;
        } else {
            self.profiles.push(profile);
        }
        Ok(())
    }

    /// Mark a profile for verification by platform
    pub fn mark_for_verification(
        &mut self,
        platform: SocialMediaPlatform,
    ) -> Result<(), DomainError> {
        if let Some(profile) = self.profiles.iter_mut().find(|p| p.platform() == platform) {
            *profile = SocialMediaMetadata::new(
                profile.platform(),
                profile.profile_name(),
                profile.profile_link(),
                true,
                profile.is_verified(),
                profile.follower_count(),
                profile.demographics().cloned(),
            )?;
            Ok(())
        } else {
            Err(DomainError::not_found(format!("{platform:?} profile")))
        }
    }
}
