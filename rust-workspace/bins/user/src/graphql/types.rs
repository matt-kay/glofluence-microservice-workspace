use std::collections::HashMap;

use async_graphql::{ID, SimpleObject};
use bin_shared_kernel::predule::SocialProfile;
use corelib::predule::User as DomainUser;
use uuid::Uuid;

#[derive(SimpleObject)]
pub struct User {
    pub id: ID,

    pub first_name: String,
    pub last_name: String,
    pub country_term_id: Uuid,

    pub social_profiles: Option<Vec<SocialProfile>>,

    pub demographics: Option<HashMap<String, Vec<String>>>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
    pub deletetion_status: Option<String>,
    pub version: u64,
}

impl From<DomainUser> for User {
    fn from(value: DomainUser) -> Self {
        Self {
            id: value.id.as_uuid().into(),
            first_name: value.first_name.as_str().to_string(),
            last_name: value.last_name.as_str().to_string(),
            country_term_id: value.country_term_id.as_uuid(),
            social_profiles: value.social_profiles.map(|v| {
                v.all()
                    .iter()
                    .map(|item| SocialProfile::from(item))
                    .collect::<Vec<SocialProfile>>()
            }),
            demographics: value.demographics.map(|v| {
                v.iter()
                    .map(|(tax_id, terms)| {
                        (tax_id.as_str(), terms.iter().map(|t| t.as_str()).collect())
                    })
                    .collect::<HashMap<String, Vec<String>>>()
            }),
            created_at: value.timestamps.created_human(),
            updated_at: value.timestamps.updated_human(),
            deleted: value.deleted.is_deleted(),
            deletetion_status: Some(value.deleted.status()),
            version: value.version,
        }
    }
}
