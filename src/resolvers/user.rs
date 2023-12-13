use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, SimpleObject)]
pub struct LibrusUser {
    pub account_id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub group_id: i32,
    pub id: i32,
    pub is_employee: bool,
}