use std::str::FromStr;

use async_graphql::*;
use corelib::predule::UserId;
use uuid::Uuid;

use crate::{graphql::types::User, setup::state::AppState};

pub struct Query;

#[Object]
impl Query {
    /// Get a single User
    #[graphql(entity)]
    async fn get_user<'ctx>(&self, ctx: &Context<'ctx>, id: ID) -> Result<Option<User>, Error> {
        let app_state = ctx
            .data::<AppState>()
            .map_err(|_| Error::new("AppState not available"))?;

        let user_service = app_state.user_service.lock().await;

        let u_id = Uuid::from_str(id.as_str())?;
        let user_id = UserId::from_uuid(u_id);

        
        let domain_user = user_service.find_by_id(&user_id).await?;

        Ok(domain_user.map(User::from))
    }

    /// Get many  User
    async fn get_users(&self) -> &'static str {
        "get users"
    }

    /// Search  User
    async fn search_users(&self) -> &'static str {
        "search users"
    }
}
