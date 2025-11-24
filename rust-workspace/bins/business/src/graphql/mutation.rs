use std::collections::HashMap;

use async_graphql::{Context, ErrorExtensions, Object, Result};
use corelib::predule::{
    BusinessDescription, BusinessFeatures, BusinessHourEntry, BusinessId, BusinessName,
    ContactInfo, DomainError, EmailAddress, ExtraFeatureKey, ExtraFeatureValue, PhoneNumber,
    PhysicalAddress, ServiceName, SocialMedia, SocialMediaLink, SocialPlatformName, Tag,
    WebsiteUrl,
};
use uuid::Uuid;

use crate::graphql::inputs::{CreateBusinessInput, UpdateBusinessInput};
use crate::graphql::types::Business;
use crate::setup::state::AppState;

pub struct Mutation;

#[Object]
impl Mutation {
    /// Create a Business
    async fn create_business<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: CreateBusinessInput,
    ) -> Result<Business> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut business_service = app_state.business_service.lock().await;

        let business_name = BusinessName::new(input.name)?;

        let business_description = input
            .description
            .map(BusinessDescription::new)
            .transpose()?;

        let business_contact_info = if let Some(ci) = input.contact_info {
            let email = ci.email.map(EmailAddress::new).transpose()?;
            let phone = ci.phone.map(PhoneNumber::new).transpose()?;
            let address = ci.address.map(PhysicalAddress::new).transpose()?;
            let website = ci.website.map(WebsiteUrl::new).transpose()?;

            Some(ContactInfo::new(email, phone, address, website))
        } else {
            None
        };

        let business_social_media = if let Some(sm) = input.social_media {
            let facebook = sm.facebook.map(SocialMediaLink::new).transpose()?;
            let instagram = sm.instagram.map(SocialMediaLink::new).transpose()?;
            let twitter = sm.twitter.map(SocialMediaLink::new).transpose()?;
            let tiktok = sm.tiktok.map(SocialMediaLink::new).transpose()?;
            let linkedin = sm.linkedin.map(SocialMediaLink::new).transpose()?;
            let youtube = sm.youtube.map(SocialMediaLink::new).transpose()?;

            // Handle social_media.other (HashMap<String, String> → HashMap<PlatformName, Link>)
            let other = if let Some(other_map) = sm.other {
                Some(
                    other_map
                        .into_iter()
                        .map(|(k, v)| {
                            Ok::<_, DomainError>((
                                SocialPlatformName::new(k)?,
                                SocialMediaLink::new(v)?,
                            ))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?,
                )
            } else {
                None
            };

            Some(SocialMedia::new(
                facebook, instagram, twitter, tiktok, linkedin, youtube, other,
            ))
        } else {
            None
        };

        let business_features = if let Some(f) = input.features {
            // hours → Option<Vec<BusinessHourEntry>>
            let hours = if let Some(hours_vec) = f.hours {
                Some(
                    hours_vec
                        .into_iter()
                        .map(|h| BusinessHourEntry::new(h.day, h.hours))
                        .collect::<Result<Vec<_>, _>>()?,
                )
            } else {
                None
            };

            // services → Option<Vec<ServiceName>>
            let services = if let Some(svc_vec) = f.services {
                Some(
                    svc_vec
                        .into_iter()
                        .map(ServiceName::new)
                        .collect::<Result<Vec<_>, _>>()?,
                )
            } else {
                None
            };

            // tags → Option<Vec<Tag>>
            let tags = if let Some(tag_vec) = f.tags {
                Some(
                    tag_vec
                        .into_iter()
                        .map(Tag::new)
                        .collect::<Result<Vec<_>, _>>()?,
                )
            } else {
                None
            };

            // extra → Option<HashMap<ExtraFeatureKey, ExtraFeatureValue>>
            let extras = if let Some(extra_map) = f.extra {
                Some(
                    extra_map
                        .into_iter()
                        .map(|(k, v)| {
                            Ok::<_, DomainError>((
                                ExtraFeatureKey::new(k)?,
                                ExtraFeatureValue::new(v)?,
                            ))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?,
                )
            } else {
                None
            };

            Some(BusinessFeatures::new(hours, services, tags, extras))
        } else {
            None
        };

        // Save business via service
        let domain_business = business_service
            .create_business(
                business_name,
                business_description,
                business_contact_info,
                business_social_media,
                business_features,
            )
            .await?;

        let business = Business::from(domain_business);

        Ok(business)
    }

    /// Update a Business
    async fn update_business<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        business_id: Uuid,
        input: UpdateBusinessInput,
    ) -> Result<Business> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut business_service = app_state.business_service.lock().await;

        let b_id = BusinessId::from_uuid(business_id);

        let business_name = input.name.map(BusinessName::new).transpose()?;
        let business_description = input
            .description
            .map(BusinessDescription::new)
            .transpose()?;

        let business_contact_info = if let Some(ci) = input.contact_info {
            let email = ci.email.map(EmailAddress::new).transpose()?;
            let phone = ci.phone.map(PhoneNumber::new).transpose()?;
            let address = ci.address.map(PhysicalAddress::new).transpose()?;
            let website = ci.website.map(WebsiteUrl::new).transpose()?;

            Some(ContactInfo::new(email, phone, address, website))
        } else {
            None
        };

        let business_social_media = if let Some(sm) = input.social_media {
            let facebook = sm.facebook.map(SocialMediaLink::new).transpose()?;
            let instagram = sm.instagram.map(SocialMediaLink::new).transpose()?;
            let twitter = sm.twitter.map(SocialMediaLink::new).transpose()?;
            let tiktok = sm.tiktok.map(SocialMediaLink::new).transpose()?;
            let linkedin = sm.linkedin.map(SocialMediaLink::new).transpose()?;
            let youtube = sm.youtube.map(SocialMediaLink::new).transpose()?;

            // Handle social_media.other (HashMap<String, String> → HashMap<PlatformName, Link>)
            let other = if let Some(other_map) = sm.other {
                Some(
                    other_map
                        .into_iter()
                        .map(|(k, v)| {
                            Ok::<_, DomainError>((
                                SocialPlatformName::new(k)?,
                                SocialMediaLink::new(v)?,
                            ))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?,
                )
            } else {
                None
            };

            Some(SocialMedia::new(
                facebook, instagram, twitter, tiktok, linkedin, youtube, other,
            ))
        } else {
            None
        };

        let business_features = if let Some(f) = input.features {
            // hours → Option<Vec<BusinessHourEntry>>
            let hours = if let Some(hours_vec) = f.hours {
                Some(
                    hours_vec
                        .into_iter()
                        .map(|h| BusinessHourEntry::new(h.day, h.hours))
                        .collect::<Result<Vec<_>, _>>()?,
                )
            } else {
                None
            };

            // services → Option<Vec<ServiceName>>
            let services = if let Some(svc_vec) = f.services {
                Some(
                    svc_vec
                        .into_iter()
                        .map(ServiceName::new)
                        .collect::<Result<Vec<_>, _>>()?,
                )
            } else {
                None
            };

            // tags → Option<Vec<Tag>>
            let tags = if let Some(tag_vec) = f.tags {
                Some(
                    tag_vec
                        .into_iter()
                        .map(Tag::new)
                        .collect::<Result<Vec<_>, _>>()?,
                )
            } else {
                None
            };

            // extra → Option<HashMap<ExtraFeatureKey, ExtraFeatureValue>>
            let extras = if let Some(extra_map) = f.extra {
                Some(
                    extra_map
                        .into_iter()
                        .map(|(k, v)| {
                            Ok::<_, DomainError>((
                                ExtraFeatureKey::new(k)?,
                                ExtraFeatureValue::new(v)?,
                            ))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?,
                )
            } else {
                None
            };

            Some(BusinessFeatures::new(hours, services, tags, extras))
        } else {
            None
        };

        // Save business via service
        let domain_business = business_service
            .update_business(
                b_id,
                business_name,
                business_description,
                business_contact_info,
                business_social_media,
                business_features,
            )
            .await?;

        let business = Business::from(domain_business);

        Ok(business)
    }

    /// Soft Delete a Business
    async fn soft_delete_business<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        business_id: Uuid,
    ) -> Result<Uuid> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut business_service = app_state.business_service.lock().await;

        let b_id = BusinessId::from_uuid(business_id);

        let domain_business = business_service.soft_delete_business(b_id).await?;

        Ok(domain_business.id.as_uuid())
    }

    /// Permanetly Delete a Business
    async fn permanetly_delete_business<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        business_id: Uuid,
    ) -> Result<&'static str> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut business_service = app_state.business_service.lock().await;

        let b_id = BusinessId::from_uuid(business_id);

        business_service.permanetly_delete_business(b_id).await?;

        Ok("deleted!")
    }
}
