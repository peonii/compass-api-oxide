use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct LibrusResource {
    pub id: i32,
    pub url: String,
}