use async_graphql::{Context, Object};
use sqlx::PgPool;
use time::OffsetDateTime;

use crate::{database::user::{User, UserSession, NewUser}, librus::{client::LibrusClient, self}};


#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
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

            let me: librus::user::APIMeResponse = librus_client.request("https://api.librus.pl/3.0/Me").await?;
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