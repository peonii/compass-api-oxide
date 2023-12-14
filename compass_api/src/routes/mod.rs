use poem::{Route, get};

pub mod graphql;

pub fn router() -> Route {
    Route::new()
        .at("/graphql", get(graphql::playground).post(graphql::graphql))
}