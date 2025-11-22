use async_graphql::{Enum, SimpleObject};
use corelib::predule::{SocialMediaMetadata, SocialMediaPlatform};

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum SocialProfilePlatform {
    Facebook,
    Instagram,
    TikTok,
    X,
    Youtube,
    LinkedIn,
}

impl From<SocialMediaPlatform> for SocialProfilePlatform {
    fn from(platform: SocialMediaPlatform) -> Self {
        match platform {
            SocialMediaPlatform::Facebook => SocialProfilePlatform::Facebook,
            SocialMediaPlatform::Instagram => SocialProfilePlatform::Instagram,
            SocialMediaPlatform::TikTok => SocialProfilePlatform::TikTok,
            SocialMediaPlatform::X => SocialProfilePlatform::X,
            SocialMediaPlatform::Youtube => SocialProfilePlatform::Youtube,
            SocialMediaPlatform::LinkedIn => SocialProfilePlatform::LinkedIn,
        }
    }
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

// From implementation for owned value
impl From<SocialMediaMetadata> for SocialProfile {
    fn from(value: SocialMediaMetadata) -> Self {
        Self::from(&value) // delegate to reference-based impl
    }
}

// From implementation for reference
impl From<&SocialMediaMetadata> for SocialProfile {
    fn from(value: &SocialMediaMetadata) -> Self {
        Self {
            platform: value.platform().into(),
            profile_name: value.profile_name().to_string(),
            profile_link: value.profile_link().to_string(),
            mark_for_verification: value.mark_for_verification(),
            is_verified: value.is_verified(),
            follower_count: value.follower_count(),
        }
    }
}
