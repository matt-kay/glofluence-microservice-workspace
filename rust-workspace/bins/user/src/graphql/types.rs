use async_graphql::SimpleObject;
use bin_shared_kernel::predule::SocialProfile;
use corelib::predule::User as DomainUser;
use uuid::Uuid;

#[derive(SimpleObject)]
pub struct User {
    pub id: Uuid,

    pub first_name: String,
    pub last_name: String,
    pub country_term_id: Uuid,

    pub social_profiles: Vec<SocialProfile>,

    // pub demographics: Demographics,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
    pub deletetion_status: Option<String>,
    pub version: u64,
}

impl From<DomainUser> for User {
    fn from(value: DomainUser) -> Self {
        Self {
            id: value.id.as_uuid(),
            first_name: value.first_name.as_str().to_string(),
            last_name: value.last_name.as_str().to_string(),
            country_term_id: value.country_term_id.as_uuid(),
            social_profiles: value
                .social_profiles
                .all()
                .iter() // iterator over references
                .map(|item| SocialProfile::from(item)) // uses &SocialMediaMetadata impl
                .collect(),
            created_at: value.timestamps.created_human(),
            updated_at: value.timestamps.updated_human(),
            deleted: value.deleted.is_deleted(),
            deletetion_status: Some(value.deleted.status()),
            version: value.version,
        }
    }
}
