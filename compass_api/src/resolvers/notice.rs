use async_graphql::{Object, Context, Result, SimpleObject, ComplexObject, dataloader::DataLoader};

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

        let mut librus_client = LibrusClient::new();
        librus_client.token = Some(user.librus_access_token.clone());

        let users: APINoticesResponse = librus_client.request("https://api.librus.pl/3.0/SchoolNotices").await?;
        let users = users.notices;

        let librus_notices = users
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