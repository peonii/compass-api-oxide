use serde::{Deserialize, Serialize};

use super::api::LibrusResource;


// #[derive(SimpleObject, Default)]
// #[graphql(complex)]
// pub struct LibrusNotice {
//     id: String,
//     start_date: String,
//     end_date: String,
//     subject: String,
//     content: String,
//     creation_date: String,
//     was_read: bool,
//     author_res: LibrusResource
// }

#[derive(Serialize, Deserialize)]
pub struct APINotice {
    #[serde(alias = "Id")]
    pub id: String,

    #[serde(alias = "StartDate")]
    pub start_date: String,

    #[serde(alias = "EndDate")]
    pub end_date: String,

    #[serde(alias = "Subject")]
    pub subject: String,

    #[serde(alias = "Content")]
    pub content: String,

    #[serde(alias = "AddedBy")]
    pub author_res: LibrusResource,

    #[serde(alias = "CreationDate")]
    pub creation_date: String,

    #[serde(alias = "WasRead")]
    pub was_read: bool,
}

#[derive(Serialize, Deserialize)]
pub struct APINoticesResponse {
    #[serde(alias = "SchoolNotices")]
    pub notices: Vec<APINotice>,
}