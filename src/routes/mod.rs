use async_graphql_poem::GraphQL;
use poem::{Route, get};
use redis::Client;
use sqlx::PgPool;

use crate::schema;

pub mod graphql;

pub fn router(pg: PgPool, redis: Client) -> Route {
    Route::new()
        .at("/graphql", get(graphql::playground).post(GraphQL::new(schema::build_schema(pg, redis))))
}