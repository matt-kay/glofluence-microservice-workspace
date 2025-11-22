use std::error::Error;

use async_graphql_axum::GraphQL;
use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::{
    routes::handlers::graphiql::graphiql,
    setup::{env::EnvConfig, schema::build_schema, state::build_state},
};

mod graphql;
mod routes;
mod setup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load env
    let env_config = EnvConfig::load();

    // build app state
    let app_state = build_state();

    // build graphql schema
    let gql_schema = build_schema(app_state);

    // setup server routes
    let routes = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(gql_schema)))
        .route("/health", get(|| async { "ok" }));

    dbg!(format!(
        "[LOG] User subgrah running at http://localhost:{}",
        env_config.server.port
    ));
    axum::serve(
        TcpListener::bind(format!("127.0.0.1:{}", env_config.server.port))
            .await
            .unwrap(),
        routes,
    )
    .await
    .unwrap();

    Ok(())
}
