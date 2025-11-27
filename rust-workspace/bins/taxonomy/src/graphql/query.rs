use std::str::FromStr;

use async_graphql::*;
use corelib::predule::TaxonomyId;
use uuid::Uuid;

use crate::{graphql::types::Taxonomy, setup::state::AppState};

pub struct Query;

#[Object]
impl Query {
    /// Get a single Taxonomy
    #[graphql(entity)]
    async fn get_term<'ctx>(&self, ctx: &Context<'ctx>, id: ID) -> Result<Option<Taxonomy>, Error> {
        let app_state = ctx
            .data::<AppState>()
            .map_err(|_| Error::new("AppState not available"))?;

        let taxonomy_service = app_state.taxonomy_service.lock().await;

        let t_id = Uuid::from_str(id.as_str())?;
        let term_id = TaxonomyId::from_uuid(t_id);
        let domain_term = taxonomy_service.find_by_id(&term_id).await?;

        Ok(domain_term.map(Taxonomy::from))
    }

    /// Get many  Taxonomy
    async fn get_taxonomy(&self) -> &'static str {
        "get taxonomy"
    }

    /// Search  Taxonomy
    async fn search_taxonomy(&self) -> &'static str {
        "search taxonomy"
    }
}
