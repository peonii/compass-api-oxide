use async_graphql::{Object, Schema, EmptyMutation, EmptySubscription};


pub struct Query {}

#[Object]
impl Query {
    async fn hello(&self) -> &'static str {
        "Pizdec pizdec."
    }
}

pub fn build_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query {}, EmptyMutation, EmptySubscription)
        .finish()
}