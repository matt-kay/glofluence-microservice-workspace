use async_graphql::{Context, ErrorExtensions, Object, Result};
use corelib::predule::{FirstName, LastName, TermId, UserId};
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

        let first_name = FirstName::new(input.first_name).map_err(|err| {
            err.extend_with(|_, e| {
                e.set("code", 400);
                e.set("message", err.to_string());
            })
        })?;

        let last_name = LastName::new(input.last_name).map_err(|err| {
            err.extend_with(|_, e| {
                e.set("code", 400);
                e.set("message", err.to_string());
            })
        })?;
        let country_term_id = TermId::from_uuid(input.country_term_id);
        // Save user via service
        let domain_user = user_service
            .create_user(first_name, last_name, country_term_id)
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

        let first_name = match input.first_name {
            Some(f) => Some(FirstName::new(f).map_err(|err| {
                err.extend_with(|_, e| {
                    e.set("code", 400);
                    e.set("message", err.to_string());
                })
            })?),
            None => None,
        };

        let last_name = match input.last_name {
            Some(l) => Some(LastName::new(l).map_err(|err| {
                err.extend_with(|_, e| {
                    e.set("code", 400);
                    e.set("message", err.to_string());
                })
            })?),
            None => None,
        };

        let country_term_id = input.country_term_id.map(|c| {
            let c_t_id = TermId::from_uuid(c);
            c_t_id
        });
        // Save user via service
        let domain_user = user_service
            .update_user(u_id, first_name, last_name, country_term_id)
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
    async fn permanetly_delete_user<'ctx>(&self, ctx: &Context<'ctx>, user_id: Uuid) -> Result<&'static str> {
        let app_state = ctx.data::<AppState>().expect("AppState not initialized");

        let mut user_service = app_state.user_service.lock().await;

        let u_id = UserId::from_uuid(user_id);

        user_service.permanetly_delete_user(u_id).await?;

        Ok("deleted!")
    }
}
