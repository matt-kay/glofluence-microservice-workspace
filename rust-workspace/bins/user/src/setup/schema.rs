use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::graphql::query::Query;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        // .enable_federation()
        .finish()
}
