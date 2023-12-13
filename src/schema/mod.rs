use async_graphql::{Object, Schema, EmptySubscription, Context};
use sqlx::PgPool;
use time::OffsetDateTime;

use crate::{database::user::{User, UserSession, NewUser}, librus::{client::LibrusClient, self, user::APIUsersResponse}, resolvers::user::LibrusUser};


pub struct Query {}

#[Object]
impl Query {
    async fn health(&self) -> &'static str {
        "Please buy obot"
    }

    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<LibrusUser>> {
        let user_id = ctx.data::<uuid::Uuid>()?;
        let pg = ctx.data::<PgPool>()?;

        let user = User::find_by_id(&pg, *user_id).await?;

        if user.is_none() {
            return Err("User not found".into());
        }

        let user = user.unwrap();

        let mut librus_client = LibrusClient::new();
        librus_client.token = Some(user.librus_access_token);

        let users: APIUsersResponse = librus_client.request("https://api.librus.pl/3.0/Users").await?;
        let users = users.users;

        let librus_users = users
            .into_iter()
            .map(|u| {
                LibrusUser {
                    account_id: u.account_id,
                    first_name: u.first_name,
                    last_name: u.last_name,
                    group_id: u.group_id,
                    id: u.id,
                    is_employee: u.is_employee,
                }
            })
            .collect::<Vec<LibrusUser>>();

        Ok(librus_users)
    }
}

pub struct Mutation {}

#[Object]
impl Mutation {
    async fn log_in(&self, ctx: &Context<'_>, email: String, password: String) -> async_graphql::Result<UserSession> {
        let pg = ctx.data::<PgPool>()?;
        let redis = ctx.data::<redis::Client>()?;

        let user = User::find_by_email(&pg, &email).await?;

        if let Some(user) = user {
            if user.password == password {
                let session = UserSession::create(&redis, user.id).await?;

                return Ok(session);
            }
        } else {
            let mut librus_client = LibrusClient::new();

            librus_client.log_in(email.clone(), password.clone()).await?;

            let me : librus::user::APIMeResponse = librus_client.request("https://api.librus.pl/3.0/Me").await?;
            let user = me.me.account;

            let user = NewUser {
                email: email.clone(),
                password: password.clone(),
                first_name: user.first_name,
                last_name: user.last_name,
                librus_access_token: librus_client.token.unwrap(),
                next_check_at: OffsetDateTime::now_utc(),
                is_test_account: false,
            };

            let user = user.create_or_update(&pg).await?;

            let session = UserSession::create(&redis, user.id).await?;

            return Ok(session);
        }

        Err("Invalid email or password".into())
    }
}

pub type CompassSchema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> CompassSchema {
    Schema::build(Query {}, Mutation {}, EmptySubscription)
        .finish()
}