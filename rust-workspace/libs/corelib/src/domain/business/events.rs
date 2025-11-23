use serde::{Deserialize, Serialize};

use crate::domain::{
    business::value_objects::{BusinessDescription, BusinessFeatures, BusinessName, ContactInfo},
    shared::{event::EventMeta, value_object::SocialMedia},
};

/// Domain events for the Business aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum BusinessDomainEvent {
    /// A new business was created
    BusinessCreated {
        meta: EventMeta,
        event_name: String,

        name: BusinessName,
        description: Option<BusinessDescription>,
        contact_info: ContactInfo,
        social_media: SocialMedia,
        features: BusinessFeatures,
    },

    /// Business name or description changed
    BusinessDetailsUpdated {
        meta: EventMeta,
        event_name: String,

        previous_name: Option<BusinessName>,
        previous_description: Option<BusinessDescription>,

        latest_name: Option<BusinessName>,
        latest_description: Option<BusinessDescription>,
    },

    /// Contact information changed
    BusinessContactUpdated {
        meta: EventMeta,
        event_name: String,

        previous: ContactInfo,
        latest: ContactInfo,
    },

    /// Social media links changed
    BusinessSocialMediaUpdated {
        meta: EventMeta,
        event_name: String,

        previous: SocialMedia,
        latest: SocialMedia,
    },

    /// Business features metadata changed
    BusinessFeaturesUpdated {
        meta: EventMeta,
        event_name: String,

        previous: BusinessFeatures,
        latest: BusinessFeatures,
    },

    /// Business permanently deleted
    BusinessDeleted { meta: EventMeta, event_name: String },

    /// Business soft-deleted (not permanently removed)
    BusinessSoftDeleted { meta: EventMeta, event_name: String },

    /// Business restored from soft-delete
    BusinessRestoredFromSoftDeleted { meta: EventMeta, event_name: String },
}
