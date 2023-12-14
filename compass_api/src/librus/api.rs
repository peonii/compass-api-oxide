use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, SimpleObject)]
pub struct LibrusResource {
    #[serde(alias = "Id")]
    pub id: i32,

    #[serde(alias = "Url")]
    pub url: String,
}

pub trait LibrusTypeSingular<T> {
    fn get(&self) -> &T;
}

pub trait LibrusTypePlural<T> {
    fn get(&self) -> &Vec<T>;
}