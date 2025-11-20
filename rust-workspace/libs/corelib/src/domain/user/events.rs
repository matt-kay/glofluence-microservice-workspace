use serde::{Deserialize, Serialize};

use crate::domain::shared::{
    event::EventMeta,
    value_object::{Demographics, SocialMediaProfiles},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum UserDomainEvent {
    UserCreated {
        meta: EventMeta,
        event_name: String,
        first_name: String,
        last_name: String,
        country_term_id: String,
    },
    UserBioUpdated {
        meta: EventMeta,
        event_name: String,
        first_name: Option<String>,
        last_name: Option<String>,
        country_term_id: Option<String>,
    },
    UserSocialProfileUpdated {
        meta: EventMeta,
        event_name: String,
        latest: SocialMediaProfiles,
    },
    UserDemographicsUpdated {
        meta: EventMeta,
        event_name: String,
        latest: Demographics,
    },
    UserDeleted {
        meta: EventMeta,
        event_name: String,
    },
}

pub trait UserEventBus: Send + Sync {
    fn publish(&self, events: &[UserDomainEvent]) -> Result<(), String>;
}
