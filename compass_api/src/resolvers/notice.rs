use async_graphql::{Object, Context, Result, SimpleObject, ComplexObject, dataloader::DataLoader, FieldError};

use crate::{librus::{notice::APINoticesResponse, client::LibrusClient}, database::user::User};

use super::user::{LibrusUser, LibrusUserLoader};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct LibrusNotice {
    id: String,
    start_date: String,
    end_date: String,
    subject: String,
    content: String,
    creation_date: String,
    was_read: bool,
    author_id: i32
}

#[ComplexObject]
impl LibrusNotice {
    async fn author(&self, ctx: &Context<'_>) -> Result<Option<LibrusUser>> {
        ctx.data_unchecked::<DataLoader<LibrusUserLoader>>()
            .load_one(self.author_id)
            .await
    }
}


#[derive(Default)]
pub struct NoticeQuery;

#[Object]
impl NoticeQuery {
    async fn notices(&self, ctx: &Context<'_>) -> Result<Vec<LibrusNotice>> {
        let user = ctx.data::<User>()?;
        let pg = ctx.data::<sqlx::PgPool>()?;

        let mut librus_client = LibrusClient::new();
        librus_client.token = Some(user.librus_access_token.clone());

        let mut notices = librus_client.request::<APINoticesResponse>("https://api.librus.pl/3.0/SchoolNotices").await;
        if notices.is_err() {
            // Try to reauth
            librus_client.log_in(user.email.clone(), user.password.clone())
                .await?;

            notices = librus_client.request::<APINoticesResponse>("https://api.librus.pl/3.0/SchoolNotices").await;

            if notices.is_err() {
                return Err(FieldError::from("Failed to fetch notices"));
            }

            User::update_token(pg, user.id, &librus_client.token.unwrap())
                .await?;
        }

        let notices = notices?.notices;

        let librus_notices = notices
            .into_iter()
            .map(|u| {
                LibrusNotice {
                    id: u.id.to_string(),
                    start_date: u.start_date,
                    end_date: u.end_date,
                    subject: u.subject,
                    content: u.content,
                    creation_date: u.creation_date,
                    was_read: u.was_read,
                    author_id: u.author_res.id,
                }
            })
            .collect::<Vec<LibrusNotice>>();

        Ok(librus_notices)
    }
}