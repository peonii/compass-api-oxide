use async_graphql_poem::GraphQL;
use poem::{Route, get};

use crate::schema;

pub mod graphql;

pub fn router() -> Route {
    Route::new()
        .at("/graphql", get(graphql::playground).post(GraphQL::new(schema::build_schema())))
}