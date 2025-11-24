use std::collections::HashMap;

use async_graphql::InputObject;
use uuid::Uuid;

use crate::predule::SocialProfilePlatform;

#[derive(InputObject)]
pub struct SocialProfileInput {
    pub platform: SocialProfilePlatform,
    pub profile_name: String,
    pub profile_link: String,
    pub mark_for_verification: bool,
    pub is_verified: bool,
    pub follower_count: u64,
    pub demographics: Option<HashMap<Uuid, Vec<Uuid>>>,
}
