use std::collections::HashMap;

use async_graphql::{Context, Object, Result};
use corelib::predule::{
    DomainError, FirstName, LastName, SocialMediaMetadata, SocialMediaProfiles, TaxonomyId, TermId,
    UserId,
};
use uuid::Uuid;

use crate::graphql::inputs::{CreateUserInput, UpdateUserInput};
use crate::graphql::types::User;
use crate::setup::state::AppState;

pub struct Mutation;

#[Object]
impl Mutation {
    /// Create a User
    async fn create_user<'ctx>(&self, ctx: &Context<'ctx>, input: CreateUserInput) -> Result<User> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut user_service = app_state.user_service.lock().await;

        let first_name = FirstName::new(input.first_name)?;

        let last_name = LastName::new(input.last_name)?;

        let country_term_id = TermId::from_uuid(input.country_term_id);
        let social_media = if let Some(sm) = input.social_profiles {
            let social_media_meta_data = sm
                .into_iter()
                .map(|v| {
                    SocialMediaMetadata::new(
                        v.platform.into(),
                        v.profile_name,
                        v.profile_link,
                        v.mark_for_verification,
                        v.is_verified,
                        v.follower_count,
                        v.demographics.map(|k| {
                            k.into_iter()
                                .map(|(o, p)| {
                                    (
                                        TaxonomyId::from_uuid(o),
                                        p.into_iter()
                                            .map(|elem| TermId::from_uuid(elem))
                                            .collect::<Vec<TermId>>(),
                                    )
                                })
                                .collect::<HashMap<TaxonomyId, Vec<TermId>>>()
                        }),
                    )
                })
                .collect::<Result<Vec<_>, DomainError>>()?; // <-- FIX HERE

            let profiles = SocialMediaProfiles::new(social_media_meta_data);
            Some(profiles)
        } else {
            None
        };

        let demographics = if let Some(demograph) = input.demographics {
            Some(
                demograph
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            TaxonomyId::from_uuid(k),
                            v.into_iter()
                                .map(|elem| TermId::from_uuid(elem))
                                .collect::<Vec<TermId>>(),
                        )
                    })
                    .collect::<HashMap<TaxonomyId, Vec<TermId>>>(),
            )
        } else {
            None
        };
        // Save user via service
        let domain_user = user_service
            .create_user(
                first_name,
                last_name,
                country_term_id,
                social_media,
                demographics,
            )
            .await?;

        let user = User::from(domain_user);

        Ok(user)
    }

    /// Update a User
    async fn update_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: Uuid,
        input: UpdateUserInput,
    ) -> Result<User> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut user_service = app_state.user_service.lock().await;

        let u_id = UserId::from_uuid(user_id);

        let first_name = input.first_name.map(FirstName::new).transpose()?;

        let last_name = input.last_name.map(LastName::new).transpose()?;

        let country_term_id = input.country_term_id.map(TermId::from_uuid);

        let social_media = if let Some(sm) = input.social_profiles {
            let social_media_meta_data = sm
                .into_iter()
                .map(|v| {
                    SocialMediaMetadata::new(
                        v.platform.into(),
                        v.profile_name,
                        v.profile_link,
                        v.mark_for_verification,
                        v.is_verified,
                        v.follower_count,
                        v.demographics.map(|k| {
                            k.into_iter()
                                .map(|(o, p)| {
                                    (
                                        TaxonomyId::from_uuid(o),
                                        p.into_iter()
                                            .map(|elem| TermId::from_uuid(elem))
                                            .collect::<Vec<TermId>>(),
                                    )
                                })
                                .collect::<HashMap<TaxonomyId, Vec<TermId>>>()
                        }),
                    )
                })
                .collect::<Result<Vec<_>, DomainError>>()?;

            let profiles = SocialMediaProfiles::new(social_media_meta_data);
            Some(profiles)
        } else {
            None
        };

        let demographics = if let Some(demograph) = input.demographics {
            Some(
                demograph
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            TaxonomyId::from_uuid(k),
                            v.into_iter()
                                .map(|elem| TermId::from_uuid(elem))
                                .collect::<Vec<TermId>>(),
                        )
                    })
                    .collect::<HashMap<TaxonomyId, Vec<TermId>>>(),
            )
        } else {
            None
        };
        // Save user via service
        let domain_user = user_service
            .update_user(
                u_id,
                first_name,
                last_name,
                country_term_id,
                social_media,
                demographics,
            )
            .await?;

        let user = User::from(domain_user);

        Ok(user)
    }

    /// Soft Delete a User
    async fn soft_delete_user<'ctx>(&self, ctx: &Context<'ctx>, user_id: Uuid) -> Result<Uuid> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut user_service = app_state.user_service.lock().await;

        let u_id = UserId::from_uuid(user_id);

        let domain_user = user_service.soft_delete_user(u_id).await?;

        Ok(domain_user.id.as_uuid())
    }

    /// Permanetly Delete a User
    async fn permanetly_delete_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: Uuid,
    ) -> Result<&'static str> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut user_service = app_state.user_service.lock().await;

        let u_id = UserId::from_uuid(user_id);

        user_service.permanetly_delete_user(u_id).await?;

        Ok("deleted!")
    }
}
