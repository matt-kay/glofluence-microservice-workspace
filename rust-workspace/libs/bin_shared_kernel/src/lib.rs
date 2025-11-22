mod graphql;

pub mod predule {
    pub use crate::graphql::types::social_media::SocialProfile;
    pub use crate::graphql::types::social_media::SocialProfilePlatform;
    pub use crate::graphql::inputs::social_media::SocialProfileInput;
}
