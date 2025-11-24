use async_graphql::{Context, Object, Result};
use corelib::predule::{TaxonomyId, TermDescription, TermId, TermName};
use uuid::Uuid;

use crate::graphql::inputs::{CreateTermInput, UpdateTermInput};
use crate::graphql::types::Term;
use crate::setup::state::AppState;

pub struct Mutation;

#[Object]
impl Mutation {
    /// Create a Term
    async fn create_term<'ctx>(&self, ctx: &Context<'ctx>, input: CreateTermInput) -> Result<Term> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut term_service = app_state.term_service.lock().await;

        let taxonomy_id = TaxonomyId::from_uuid(input.taxonomy_id);

        let parent_id = input.parent_id.map(TermId::from_uuid);

        let name = TermName::new(input.name)?;

        let description = input.description.map(TermDescription::new).transpose()?;

        let visible = input.visible;
        // Save term via service
        let domain_term = term_service
            .create_term(taxonomy_id, parent_id, name, visible, description)
            .await?;

        let term = Term::from(domain_term);

        Ok(term)
    }

    /// Update a Term
    async fn update_term<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        term_id: Uuid,
        input: UpdateTermInput,
    ) -> Result<Term> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut term_service = app_state.term_service.lock().await;

        let t_id = TermId::from_uuid(term_id);

        let taxonomy_id = input.taxonomy_id.map(TaxonomyId::from_uuid);

        let parent_id = input.parent_id.map(TermId::from_uuid);

        let name = input.name.map(TermName::new).transpose()?;

        let description = input.description.map(TermDescription::new).transpose()?;

        let visible = input.visible.map(|v| v);
        // Save term via service
        let domain_term = term_service
            .update_term(t_id, taxonomy_id, parent_id, name, visible, description)
            .await?;

        let term = Term::from(domain_term);

        Ok(term)
    }

    /// Soft Delete a Term
    async fn soft_delete_term<'ctx>(&self, ctx: &Context<'ctx>, term_id: Uuid) -> Result<Uuid> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut term_service = app_state.term_service.lock().await;

        let t_id = TermId::from_uuid(term_id);

        let domain_term = term_service.soft_delete_term(t_id).await?;

        Ok(domain_term.id.as_uuid())
    }

    /// Permanetly Delete a Term
    async fn permanetly_delete_term<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        term_id: Uuid,
    ) -> Result<&'static str> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut term_service = app_state.term_service.lock().await;

        let t_id = TermId::from_uuid(term_id);

        term_service.permanetly_delete_term(t_id).await?;

        Ok("deleted!")
    }
}
