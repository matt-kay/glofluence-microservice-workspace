use async_graphql::SimpleObject;
use bin_shared_kernel::predule::SocialProfile;
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
    pub deleted_at: Option<String>,
    pub version: u64,
}
