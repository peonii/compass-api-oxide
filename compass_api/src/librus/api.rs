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

// From librus_structs!(APIUser, user, users);
// Generate the following:
// #[derive(LibrusSingular, Serialize, Deserialize)]
// pub struct APIUserResponse {
//     pub user: APIUser,
// }
//
// #[derive(LibrusPlural, Serialize, Deserialize)]
// pub struct APIUsersResponse {
//     pub users: Vec<APIUser>,
// }

#[macro_export]
macro_rules! librus_structs {
    ($str:ident, $sstr:ident, $pstr:ident, $sing:ident, $singu:expr, $plr:ident, $plru:expr) => {
        #[derive(LibrusSingular, Serialize, Deserialize)]
        pub struct $sstr {
            #[serde(alias = $singu)]
            pub $sing: $str,
        }

        #[derive(LibrusPlural, Serialize, Deserialize)]
        pub struct $pstr {
            #[serde(alias = $plru)]
            pub $plr: Vec<$str>,
        }
    };
}