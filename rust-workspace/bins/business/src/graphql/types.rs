use std::collections::HashMap;

use async_graphql::ID;
use async_graphql::SimpleObject;
use corelib::predule::Business as DomainBusiness;
use corelib::predule::BusinessFeatures as DomainBusinessFeatures;
use corelib::predule::BusinessHourEntry as DomainBusinessHourEntry;
use corelib::predule::ContactInfo as DomainContactInfo;
use corelib::predule::SocialMedia as DomainSocialMedia;


#[derive(SimpleObject)]
pub struct ContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
}

#[derive(SimpleObject)]
pub struct SocialMedia {
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub twitter: Option<String>,
    pub tiktok: Option<String>,
    pub linkedin: Option<String>,
    pub youtube: Option<String>,
    pub other: HashMap<String, String>,
}

#[derive(SimpleObject)]
pub struct BusinessHourEntry {
    day: String,
    hours: String,
}

#[derive(SimpleObject)]
pub struct BusinessFeatures {
    /// Example: Mon -> "9am–5pm"
    pub hours: Option<Vec<BusinessHourEntry>>,

    /// Services provided by the business
    pub services: Vec<String>,

    /// Descriptive tags
    pub tags: Vec<String>,

    /// Custom metadata
    pub extra: HashMap<String, String>,
}

#[derive(SimpleObject)]
pub struct Business {
    pub id: ID,

    pub name: String,
    pub description: Option<String>,
    pub contact_info: Option<ContactInfo>,
    pub social_media: Option<SocialMedia>,
    pub features: Option<BusinessFeatures>,

    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
    pub deletetion_status: Option<String>,
    pub version: u64,
}

impl From<DomainContactInfo> for ContactInfo {
    fn from(value: DomainContactInfo) -> Self {
        Self {
            email: value.email.map(|v| v.as_str().to_string()),
            phone: value.phone.map(|v| v.as_str().to_string()),
            address: value.address.map(|v| v.as_str().to_string()),
            website: value.website.map(|v| v.as_str().to_string()),
        }
    }
}
impl From<DomainSocialMedia> for SocialMedia {
    fn from(value: DomainSocialMedia) -> Self {
        Self {
            facebook: value.facebook.map(|v| v.as_str().to_string()),
            instagram: value.instagram.map(|v| v.as_str().to_string()),
            twitter: value.twitter.map(|v| v.as_str().to_string()),
            tiktok: value.tiktok.map(|v| v.as_str().to_string()),
            linkedin: value.linkedin.map(|v| v.as_str().to_string()),
            youtube: value.youtube.map(|v| v.as_str().to_string()),
            other: value
                .other
                .into_iter()
                .map(|v| {
                    v.iter()
                        .map(|(platform, link)| {
                            (platform.as_str().to_string(), link.as_str().to_string())
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl From<DomainBusinessHourEntry> for BusinessHourEntry {
    fn from(value: DomainBusinessHourEntry) -> Self {
        Self {
            day: value.day().to_string(),
            hours: value.hours().to_string(),
        }
    }
}

impl From<DomainBusinessFeatures> for BusinessFeatures {
    fn from(value: DomainBusinessFeatures) -> Self {
        Self {
            hours: value
                .hours
                .map(|v| v.into_iter().map(|p| p.into()).collect()),
            services: value
                .services
                .into_iter()
                .map(|v| v.iter().map(|p| p.as_str().to_string()).collect())
                .collect(),
            tags: value
                .tags
                .into_iter()
                .map(|v| v.iter().map(|p| p.as_str().to_string()).collect())
                .collect(),
            extra: value
                .extra
                .into_iter()
                .map(|v| {
                    v.iter()
                        .map(|p| (p.0.as_str().to_string(), p.1.as_str().to_string()))
                        .collect()
                })
                .collect(),
        }
    }
}

impl From<DomainBusiness> for Business {
    fn from(value: DomainBusiness) -> Self {
        Self {
            id: value.id.as_uuid().into(),
            name: value.name.as_str().to_string(),
            description: value.description.map(|v| v.as_str().to_string()),
            contact_info: value.contact_info.map(|v| v.into()),
            social_media: value.social_media.map(|v| v.into()),
            features: value.features.map(|v| v.into()),

            created_at: value.timestamps.created_human(),
            updated_at: value.timestamps.updated_human(),
            deleted: value.deleted.is_deleted(),
            deletetion_status: Some(value.deleted.status()),
            version: value.version,
        }
    }
}
