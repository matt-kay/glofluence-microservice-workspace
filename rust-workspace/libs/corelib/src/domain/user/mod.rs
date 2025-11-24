pub mod events;
pub mod ports;
pub mod value_object;

use crate::domain::{
    shared::{
        event::EventMeta,
        value_object::{Deleted, Demographics, EventId, OcurredAt, SocialMediaProfiles, Timestamp},
    },
    term::value_objects::TermId,
    user::{
        events::UserDomainEvent,
        value_object::{FirstName, LastName, UserId},
    },
};

/// Represents a user in the system
///
/// # Fields
/// - `id`: Unique identifier for the user.
/// - `first_name`: First name of user.
/// - `last_name`: Last name of user.
/// - `country_id`: The reference identifier for a `Term`.
/// - `social_media_platforms`: List of social media platforms of user.
/// - `demographics`: List of demographics of user.
/// - `timestamps`: Timestamp of moment of creation and update.
/// - `deleted`: Boolean to indicate if user is deleted or not and Timestamp of moment of deletion.
/// - `version`: optimistic concurrency
#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,

    pub first_name: FirstName,
    pub last_name: LastName,
    pub country_term_id: TermId,

    pub social_profiles: Option<SocialMediaProfiles>,
    pub demographics: Option<Demographics>,

    pub timestamps: Timestamp,
    pub deleted: Deleted,

    pub version: u64,

    pending_events: Vec<UserDomainEvent>,
}

impl User {
    pub fn new(
        id: UserId,
        first_name: FirstName,
        last_name: LastName,
        country_term_id: TermId,
        social_profiles: Option<SocialMediaProfiles>,
        demographics: Option<Demographics>,
    ) -> Self {
        let mut user = User {
            id,
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            country_term_id: country_term_id.clone(),
            social_profiles,
            demographics,
            version: 0,
            timestamps: Timestamp::new(),
            deleted: Deleted::new(),
            pending_events: Vec::new(),
        };

        user.pending_events.push(UserDomainEvent::UserCreated {
            meta: EventMeta {
                event_id: EventId::new(),
                occurred_at: OcurredAt::now(),
                aggregate_id: id.as_str(),
                aggregate_version: 1,
            },
            event_name: "user.created".to_owned(),
            first_name: first_name.as_str().to_owned(),
            last_name: last_name.as_str().to_owned(),
            country_term_id: country_term_id.as_str().to_owned(),
        });

        user
    }
    pub fn set_first_name(&mut self, first: FirstName) {
        self.first_name = first.clone();
        self.touch();
        self.pending_events.push(UserDomainEvent::UserBioUpdated {
            meta: self.next_meta(),
            event_name: "user.bio.update".to_owned(),
            first_name: Some(first.as_str().to_owned()),
            last_name: None,
            country_term_id: None,
        });
    }

    pub fn set_last_name(&mut self, last: LastName) {
        self.last_name = last.clone();
        self.touch();
        self.pending_events.push(UserDomainEvent::UserBioUpdated {
            meta: self.next_meta(),
            event_name: "user.bio.update".to_owned(),
            first_name: None,
            last_name: Some(last.as_str().to_owned()),
            country_term_id: None,
        });
    }

    pub fn set_country(&mut self, country_id: TermId) {
        self.country_term_id = country_id.clone();
        self.touch();
        self.pending_events.push(UserDomainEvent::UserBioUpdated {
            meta: self.next_meta(),
            event_name: "user.bio.update".to_owned(),
            first_name: None,
            last_name: None,
            country_term_id: Some(country_id.as_str().to_owned()),
        });
    }
    pub fn set_social_profiles(&mut self, social_profiles: SocialMediaProfiles) {
        self.social_profiles = Some(social_profiles.clone());
        self.touch();
        self.pending_events
            .push(UserDomainEvent::UserSocialProfileUpdated {
                meta: self.next_meta(),
                event_name: "user.social_media_profile.updated".to_owned(),
                latest: social_profiles.to_owned(),
            });
    }

    pub fn set_demographics(&mut self, demographics: Demographics) {
        self.demographics = Some(demographics.clone());
        self.touch();
        self.pending_events
            .push(UserDomainEvent::UserDemographicsUpdated {
                meta: self.next_meta(),
                event_name: "user.demographics.updated".to_owned(),
                latest: demographics.to_owned(),
            });
    }
    pub fn mark_as_deleted(&mut self) {
        self.deleted.mark_deleted();
        self.touch();
        self.pending_events.push(UserDomainEvent::UserSoftDeleted {
            meta: self.next_meta(),
            event_name: "user.soft_deleted".to_owned(),
        });
    }

    pub fn restore_from_deleted(&mut self) {
        self.deleted.restore();
        self.touch();
        self.pending_events
            .push(UserDomainEvent::UserRestoredFromSoftDeleted {
                meta: self.next_meta(),
                event_name: "user.restored_from_soft_deleted".to_owned(),
            });
    }

    fn next_meta(&self) -> EventMeta {
        EventMeta {
            event_id: EventId::new(),
            occurred_at: OcurredAt::now(),
            aggregate_id: self.id.as_str(),
            aggregate_version: self.version + 1,
        }
    }

    pub fn take_events(&mut self) -> Vec<UserDomainEvent> {
        std::mem::take(&mut self.pending_events)
    }

    fn touch(&mut self) {
        self.timestamps.touch();
        self.version += 1;
    }
}
