use serde::{Serialize, Deserialize};

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