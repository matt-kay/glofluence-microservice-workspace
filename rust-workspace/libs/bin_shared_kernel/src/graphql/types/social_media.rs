use std::collections::HashMap;

use async_graphql::{Enum, SimpleObject};
use corelib::predule::{SocialMediaMetadata, SocialMediaPlatform};
use uuid::Uuid;

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

impl From<SocialProfilePlatform> for SocialMediaPlatform {
    fn from(platform: SocialProfilePlatform) -> Self {
        match platform {
            SocialProfilePlatform::Facebook => SocialMediaPlatform::Facebook,
            SocialProfilePlatform::Instagram => SocialMediaPlatform::Instagram,
            SocialProfilePlatform::TikTok => SocialMediaPlatform::TikTok,
            SocialProfilePlatform::X => SocialMediaPlatform::X,
            SocialProfilePlatform::Youtube => SocialMediaPlatform::Youtube,
            SocialProfilePlatform::LinkedIn => SocialMediaPlatform::LinkedIn,
        }
    }
}

#[derive(SimpleObject)]
pub struct SocialProfile {
    pub platform: SocialProfilePlatform,
    pub profile_name: String,
    pub profile_link: String,
    pub mark_for_verification: bool,
    pub is_verified: bool,
    pub follower_count: u64,
    pub demographics: Option<HashMap<Uuid, Vec<Uuid>>>,
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
            demographics: value.demographics().map(|v| {
                v.iter()
                    .map(|(tax_id, terms)| {
                        (
                            tax_id.as_uuid(),
                            terms.iter().map(|t| t.as_uuid()).collect(),
                        )
                    })
                    .collect::<HashMap<Uuid, Vec<Uuid>>>()
            }),
        }
    }
}
