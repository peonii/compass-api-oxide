use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use poem::{IntoResponse, handler, web::Html};

#[handler]
pub fn playground() -> impl IntoResponse {
    let cfg = 
        GraphQLPlaygroundConfig::new("/graphql")
            .title("Compass API Explorer");

    Html(playground_source(cfg))
}