pub mod events;
pub mod ports;
pub mod value_objects;

use crate::domain::{
    business::{
        events::BusinessDomainEvent,
        value_objects::{
            BusinessDescription, BusinessFeatures, BusinessId, BusinessName, ContactInfo,
        },
    },
    shared::{
        event::EventMeta,
        value_object::{Deleted, EventId, OcurredAt, SocialMedia, Timestamp},
    },
};

#[derive(Debug, Clone)]
pub struct Business {
    pub id: BusinessId,

    pub name: BusinessName,
    pub description: Option<BusinessDescription>,
    pub contact_info: Option<ContactInfo>,
    pub social_media: Option<SocialMedia>,
    pub features: Option<BusinessFeatures>,

    pub timestamps: Timestamp,
    pub deleted: Deleted,

    pub version: u64,

    pending_events: Vec<BusinessDomainEvent>,
}

impl Business {
    pub fn new(
        id: BusinessId,

        name: BusinessName,
        description: Option<BusinessDescription>,
        contact_info: Option<ContactInfo>,
        social_media: Option<SocialMedia>,
        features: Option<BusinessFeatures>,
    ) -> Self {
        let mut business = Business {
            id,
            name: name.clone(),
            description: description.clone(),
            contact_info: contact_info.clone(),
            social_media: social_media.clone(),
            features: features.clone(),
            version: 0,
            timestamps: Timestamp::new(),
            deleted: Deleted::new(),
            pending_events: Vec::new(),
        };

        business
            .pending_events
            .push(BusinessDomainEvent::BusinessCreated {
                meta: EventMeta {
                    event_id: EventId::new(),
                    occurred_at: OcurredAt::now(),
                    aggregate_id: id.as_str(),
                    aggregate_version: 1,
                },
                event_name: "business.created".to_owned(),
                name: name.to_owned(),
                description: description.to_owned(),
                contact_info: contact_info.to_owned(),
                social_media: social_media.to_owned(),
                features: features.to_owned(),
            });

        business
    }

    pub fn set_name(&mut self, name: BusinessName) {
        let previous_name = name.clone();
        self.name = name.clone();
        self.touch();
        self.pending_events
            .push(BusinessDomainEvent::BusinessDetailsUpdated {
                meta: self.next_meta(),
                event_name: "business.details.update".to_owned(),
                previous_name: Some(previous_name.to_owned()),
                latest_name: Some(name.to_owned()),
                previous_description: None,
                latest_description: None,
            });
    }

    pub fn set_description(&mut self, description: Option<BusinessDescription>) {
        let previous_description = description.clone();
        self.description = description.clone();
        self.touch();
        self.pending_events
            .push(BusinessDomainEvent::BusinessDetailsUpdated {
                meta: self.next_meta(),
                event_name: "business.details.update".to_owned(),
                previous_name: None,
                latest_name: None,
                previous_description: previous_description,
                latest_description: description.to_owned(),
            });
    }

    pub fn set_contact_info(&mut self, contact_info: Option<ContactInfo>) {
        let previous_contact_info = contact_info.clone();
        self.contact_info = contact_info.clone();
        self.touch();
        self.pending_events
            .push(BusinessDomainEvent::BusinessContactUpdated {
                meta: self.next_meta(),
                event_name: "business.contact.update".to_owned(),
                previous: previous_contact_info,
                latest: contact_info,
            });
    }

    pub fn set_social_media(&mut self, social_media: Option<SocialMedia>) {
        let previous_social_media = social_media.clone();
        self.social_media = social_media.clone();
        self.touch();
        self.pending_events
            .push(BusinessDomainEvent::BusinessSocialMediaUpdated {
                meta: self.next_meta(),
                event_name: "business.social_media.update".to_owned(),
                previous: previous_social_media,
                latest: social_media,
            });
    }

    pub fn set_features(&mut self, features: Option<BusinessFeatures>) {
        let previous_features = features.clone();
        self.features = features.clone();
        self.touch();
        self.pending_events
            .push(BusinessDomainEvent::BusinessFeaturesUpdated {
                meta: self.next_meta(),
                event_name: "business.features.update".to_owned(),
                previous: previous_features,
                latest: features,
            });
    }

    pub fn mark_as_deleted(&mut self) {
        self.deleted.mark_deleted();
        self.touch();
        self.pending_events
            .push(BusinessDomainEvent::BusinessSoftDeleted {
                meta: self.next_meta(),
                event_name: "business.soft_deleted".to_owned(),
            });
    }

    pub fn restore_from_deleted(&mut self) {
        self.deleted.restore();
        self.touch();
        self.pending_events
            .push(BusinessDomainEvent::BusinessRestoredFromSoftDeleted {
                meta: self.next_meta(),
                event_name: "business.restored_from_soft_deleted".to_owned(),
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

    pub fn take_events(&mut self) -> Vec<BusinessDomainEvent> {
        std::mem::take(&mut self.pending_events)
    }

    fn touch(&mut self) {
        self.timestamps.touch();
        self.version += 1;
    }
}
