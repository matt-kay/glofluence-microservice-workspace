use async_graphql::InputObject;
use bin_shared_kernel::predule::SocialProfileInput;
use uuid::Uuid;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub first_name: String,
    pub last_name: String,
    pub country_term_id: Uuid,
    pub social_profiles: Vec<SocialProfileInput>,
    pub demographics: String,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country_term_id: Option<Uuid>,
    pub social_profiles: Option<Vec<SocialProfileInput>>,
    pub demographics: Option<String>,
}
