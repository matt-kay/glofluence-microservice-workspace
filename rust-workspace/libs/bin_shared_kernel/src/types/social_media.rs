use async_graphql::{Enum, SimpleObject};

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum SocialProfilePlatform {
    Facebook,
    Instagram,
    TikTok,
    X,
    Youtube,
    LinkedIn,
}

#[derive(SimpleObject)]
pub struct SocialProfile {
    platform: SocialProfilePlatform,
    profile_name: String,
    profile_link: String,
    mark_for_verification: bool,
    is_verified: bool,
    follower_count: u64,
    // demographics: Option<Demographics>,
}
