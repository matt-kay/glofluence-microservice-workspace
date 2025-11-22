use async_graphql::*;
use corelib::predule::TermId;
use uuid::Uuid;

use crate::{graphql::types::Term, setup::state::AppState};

pub struct Query;

#[Object]
impl Query {
    /// Get a single Term
    async fn get_term<'ctx>(&self, ctx: &Context<'ctx>, id: Uuid) -> Result<Option<Term>, Error> {
        let app_state = ctx
            .data::<AppState>()
            .map_err(|_| Error::new("AppState not available"))?;

        let term_service = app_state.term_service.lock().await;

        let term_id = TermId::from_uuid(id);
        let domain_term = term_service.find_by_id(&term_id).await?;

        Ok(domain_term.map(Term::from))
    }

    /// Get many  Term
    async fn get_terms(&self) -> &'static str {
        "get terms"
    }

    /// Search  Term
    async fn search_terms(&self) -> &'static str {
        "search terms"
    }
}
