use async_graphql::{http::{playground_source, GraphQLPlaygroundConfig}, dataloader::DataLoader};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use poem::{IntoResponse, handler, web::{Html, Data}, http::HeaderMap};
use sqlx::PgPool;
use redis::Client;

use crate::{schema::CompassSchema, database::user::{UserSession, User}, resolvers::user::LibrusUserLoader};

fn get_token_from_headers(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("Authorization")?;

    let auth_header = auth_header.to_str().ok()?;

    let token = auth_header.strip_prefix("Bearer ")?;

    Some(token.to_owned())
}

#[handler]
pub async fn graphql(
    schema: Data<&CompassSchema>,
    pg: Data<&PgPool>,
    redis: Data<&Client>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.0;

    if let Some(token) = get_token_from_headers(headers) {
        let session = UserSession::find_by_token(redis.0, &token).await.unwrap();

        if let Some(session) = session {
            let user = session.user_id;
            let user = User::find_by_id(pg.0, user).await.unwrap().unwrap();
            let at = user.librus_access_token.clone();

            req = req
                .data(user)
                .data(DataLoader::new(LibrusUserLoader::new(at.clone()), tokio::spawn))
        }
    }

    req = req.data(pg.clone()).data(redis.clone());

    let result: async_graphql::Response = schema.execute(req).await;

    result.into()
}

#[handler]
pub fn playground() -> impl IntoResponse {
    let cfg = 
        GraphQLPlaygroundConfig::new("/graphql")
            .title("Compass API Explorer");

    Html(playground_source(cfg))
}