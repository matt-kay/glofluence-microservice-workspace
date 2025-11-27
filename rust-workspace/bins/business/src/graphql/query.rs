use std::str::FromStr;

use async_graphql::*;
use corelib::predule::BusinessId;
use uuid::Uuid;

use crate::{graphql::types::Business, setup::state::AppState};

pub struct Query;

#[Object]
impl Query {
    /// Get a single Business Entity
    #[graphql(entity)]
    async fn by_id<'ctx>(&self, ctx: &Context<'ctx>, id: ID) -> Result<Option<Business>, Error> {
        let app_state = ctx
            .data::<AppState>()
            .map_err(|_| Error::new("AppState not available"))?;

        let business_service = app_state.business_service.lock().await;

        let b_id = Uuid::from_str(id.as_str())?;
        let business_id = BusinessId::from_uuid(b_id);
        let domain_business = business_service.find_by_id(&business_id).await?;

        Ok(domain_business.map(Business::from))
    }
    /// Get a single Business
    async fn get_business<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: ID,
    ) -> Result<Option<Business>, Error> {
        let app_state = ctx
            .data::<AppState>()
            .map_err(|_| Error::new("AppState not available"))?;

        let business_service = app_state.business_service.lock().await;

        let b_id = Uuid::from_str(id.as_str())?;
        let business_id = BusinessId::from_uuid(b_id);
        let domain_business = business_service.find_by_id(&business_id).await?;

        Ok(domain_business.map(Business::from))
    }

    /// Get many  Business
    async fn get_businesss(&self) -> &'static str {
        "get businesss"
    }

    /// Search  Business
    async fn search_businesss(&self) -> &'static str {
        "search businesss"
    }
}
