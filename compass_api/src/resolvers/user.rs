use std::collections::HashMap;

use async_graphql::{SimpleObject, Object, Context, dataloader::Loader, FieldError};
use serde::{Serialize, Deserialize};

use crate::{database::user::User, librus::{client::LibrusClient, user::{APIUsersResponse, APIUser, APIUserResponse}}};

#[derive(Serialize, Deserialize, SimpleObject, Clone)]
pub struct LibrusUser {
    pub account_id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub group_id: i32,
    pub id: i32,
    pub is_employee: bool,
}


#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<LibrusUser>> {
        let user = ctx.data::<User>()?;

        let mut librus_client = LibrusClient::new();
        librus_client.token = Some(user.librus_access_token.clone());

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

pub struct LibrusUserLoader {
    auth_token: String,
}

impl LibrusUserLoader {
    pub fn new(auth_token: String) -> Self {
        Self {
            auth_token,
        }
    }
}

#[async_trait::async_trait]
impl Loader<i32> for LibrusUserLoader {
    type Value = LibrusUser;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let mut librus_client = LibrusClient::new();
        librus_client.token = Some(self.auth_token.clone());

        let mut users = HashMap::new();

        librus_client.fetch_resources::<APIUser, APIUserResponse, APIUsersResponse>(keys.to_vec(), "https://api.librus.pl/3.0/Users/").await?
            .into_iter()
            .for_each(|u| {
                users.insert(u.id, LibrusUser {
                    account_id: u.account_id,
                    first_name: u.first_name,
                    last_name: u.last_name,
                    group_id: u.group_id,
                    id: u.id,
                    is_employee: u.is_employee,
                });
            });

        Ok(users)
    }
}