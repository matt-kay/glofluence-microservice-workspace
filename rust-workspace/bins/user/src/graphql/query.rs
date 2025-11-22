use async_graphql::*;
use corelib::predule::UserId;
use uuid::Uuid;

use crate::{graphql::types::User, setup::state::AppState};

pub struct Query;

#[Object]
impl Query {
    /// Get a single User
    async fn get_user<'ctx>(&self, ctx: &Context<'ctx>, id: Uuid) -> Option<User> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let user_service = app_state.user_service.lock().await;

        let user_id = UserId::from_uuid(id);
        let domain_user = user_service.find_by_id(&user_id).await.ok()?;

        domain_user.map(User::from)
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
