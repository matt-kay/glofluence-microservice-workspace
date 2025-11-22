use async_graphql::InputObject;

use crate::predule::SocialProfilePlatform;

#[derive(InputObject)]
pub struct SocialProfileInput {
    platform: SocialProfilePlatform,
    profile_name: String,
    profile_link: String,
    mark_for_verification: bool,
}

// impl From<SocialProfileInput> for SocialProfile {
//     fn from(input: SocialProfileInput) -> Self {
//         SocialProfile {
//             platform: input.platform,
//             profile_name: input.profile_name,
//             profile_link: input.profile_link,
//             mark_for_verification: input.mark_for_verification,
//             is_verified: false,
//             follower_count: 0,
//         }
//     }
// }
