use async_graphql::{Context, ErrorExtensions, Object, Result};
use corelib::predule::{TaxonomyDescription, TaxonomyId, TaxonomyName};
use uuid::Uuid;

use crate::graphql::inputs::{CreateTaxonomyInput, UpdateTaxonomyInput};
use crate::graphql::types::Taxonomy;
use crate::setup::state::AppState;

pub struct Mutation;

#[Object]
impl Mutation {
    /// Create a Taxonomy
    async fn create_taxonomy<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: CreateTaxonomyInput,
    ) -> Result<Taxonomy> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut taxonomy_service = app_state.taxonomy_service.lock().await;

        let parent_id = input.parent_id.map(|v| TaxonomyId::from_uuid(v));

        let name = TaxonomyName::new(input.name).map_err(|err| {
            err.extend_with(|_, e| {
                e.set("code", 400);
                e.set("message", err.to_string());
            })
        })?;

        let description = match input.description {
            Some(v) => Some(TaxonomyDescription::new(v).map_err(|err| {
                err.extend_with(|_, e| {
                    e.set("code", 400);
                    e.set("message", err.to_string());
                })
            })?),
            None => None,
        };
        let visible = input.visible;
        // Save taxonomy via service
        let domain_taxonomy = taxonomy_service
            .create_taxonomy(parent_id, name, visible, description)
            .await?;

        let taxonomy = Taxonomy::from(domain_taxonomy);

        Ok(taxonomy)
    }

    /// Update a Taxonomy
    async fn update_taxonomy<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        taxonomy_id: Uuid,
        input: UpdateTaxonomyInput,
    ) -> Result<Taxonomy> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut taxonomy_service = app_state.taxonomy_service.lock().await;

        let t_id = TaxonomyId::from_uuid(taxonomy_id);

        let parent_id = input.parent_id.map(|v| TaxonomyId::from_uuid(v));

        let name = match input.name {
            Some(v) => Some(TaxonomyName::new(v).map_err(|err| {
                err.extend_with(|_, e| {
                    e.set("code", 400);
                    e.set("message", err.to_string());
                })
            })?),
            None => None,
        };

        let description = match input.description {
            Some(v) => Some(TaxonomyDescription::new(v).map_err(|err| {
                err.extend_with(|_, e| {
                    e.set("code", 400);
                    e.set("message", err.to_string());
                })
            })?),
            None => None,
        };
        let visible = input.visible.map(|v| v);
        // Save taxonomy via service
        let domain_taxonomy = taxonomy_service
            .update_taxonomy(t_id, parent_id, name, visible, description)
            .await?;

        let taxonomy = Taxonomy::from(domain_taxonomy);

        Ok(taxonomy)
    }

    /// Soft Delete a Taxonomy
    async fn soft_delete_taxonomy<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        taxonomy_id: Uuid,
    ) -> Result<Uuid> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut taxonomy_service = app_state.taxonomy_service.lock().await;

        let t_id = TaxonomyId::from_uuid(taxonomy_id);

        let domain_taxonomy = taxonomy_service.soft_delete_taxonomy(t_id).await?;

        Ok(domain_taxonomy.id.as_uuid())
    }

    /// Permanetly Delete a Taxonomy
    async fn permanetly_delete_taxonomy<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        taxonomy_id: Uuid,
    ) -> Result<&'static str> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut taxonomy_service = app_state.taxonomy_service.lock().await;

        let t_id = TaxonomyId::from_uuid(taxonomy_id);

        taxonomy_service.permanetly_delete_taxonomy(t_id).await?;

        Ok("deleted!")
    }
}
