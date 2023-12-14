use serde::{Serialize, Deserialize};
use compass_proc_macros::{LibrusSingular, LibrusPlural};
use super::api::{LibrusTypePlural, LibrusTypeSingular};

use crate::librus_structs;


#[derive(Serialize, Deserialize)]
pub struct UserMeResponse {
    #[serde(alias = "Id")]
    pub id: i32,

    #[serde(alias = "UserId")]
    pub user_id: i32,

    #[serde(alias = "FirstName")]
    pub first_name: String,

    #[serde(alias = "LastName")]
    pub last_name: String,

    #[serde(alias = "Email")]
    pub email: String,

    #[serde(alias = "Login")]
    pub login: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserMe {
    #[serde(alias = "Account")]
    pub account: UserMeResponse,
}

#[derive(Serialize, Deserialize)]
pub struct APIMeResponse {
    #[serde(alias = "Me")]
    pub me: UserMe,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct APIUser {
    #[serde(alias = "AccountId")]
    pub account_id: String,

    #[serde(alias = "FirstName")]
    pub first_name: Option<String>,

    #[serde(alias = "LastName")]
    pub last_name: Option<String>,

    #[serde(alias = "GroupId")]
    pub group_id: i32,

    #[serde(alias = "Id")]
    pub id: i32,

    #[serde(alias = "IsEmployee")]
    pub is_employee: bool,
}

librus_structs!(APIUser, APIUserResponse, APIUsersResponse, user, "User", users, "Users");