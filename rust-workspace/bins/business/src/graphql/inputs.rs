use std::collections::HashMap;

use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ContactInfoInput {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
}

#[derive(InputObject)]
pub struct SocialMediaInput {
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub twitter: Option<String>,
    pub tiktok: Option<String>,
    pub linkedin: Option<String>,
    pub youtube: Option<String>,
    pub other: Option<HashMap<String, String>>,
}

#[derive(InputObject)]
pub struct BusinessHourEntryInput {
    pub day: String,
    pub hours: String,
}

#[derive(InputObject)]
pub struct BusinessFeaturesInput {
    /// Example: Mon -> "9am–5pm"
    pub hours: Option<Vec<BusinessHourEntryInput>>,

    /// Services provided by the business
    pub services: Option<Vec<String>>,

    /// Descriptive tags
    pub tags: Option<Vec<String>>,

    /// Custom metadata
    pub extra: Option<HashMap<String, String>>,
}

#[derive(InputObject)]
pub struct CreateBusinessInput {
    pub name: String,
    pub description: Option<String>,
    pub contact_info: Option<ContactInfoInput>,
    pub social_media: Option<SocialMediaInput>,
    pub features: Option<BusinessFeaturesInput>,
}

#[derive(InputObject)]
pub struct UpdateBusinessInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub contact_info: Option<ContactInfoInput>,
    pub social_media: Option<SocialMediaInput>,
    pub features: Option<BusinessFeaturesInput>,
}
