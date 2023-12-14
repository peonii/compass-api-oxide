use async_graphql::{Schema, EmptySubscription, MergedObject};

use crate::resolvers::{user::UserQuery, health::HealthQuery, auth::AuthMutation, notice::NoticeQuery};

#[derive(MergedObject, Default)]
pub struct Query(HealthQuery, UserQuery, NoticeQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(AuthMutation);

pub type CompassSchema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> CompassSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .finish()
}