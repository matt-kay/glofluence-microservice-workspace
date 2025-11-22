use async_graphql::{EmptySubscription, Schema};

use crate::{
    graphql::{mutation::Mutation, query::Query},
    setup::state::AppState,
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(app_state: AppState) -> AppSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        // .enable_federation()
        .data(app_state)
        .finish()
}
