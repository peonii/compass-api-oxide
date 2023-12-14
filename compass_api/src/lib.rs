#![feature(concat_idents)]
use std::env;

use poem::{Server, listener::TcpListener, EndpointExt, middleware::AddData};
use schema::build_schema;

pub mod schema;
pub mod routes;
pub mod database;
pub mod librus;
pub mod resolvers;

pub async fn serve() -> Result<(), std::io::Error> {
    let pg = database::connect().await.unwrap();
    let redis = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();

    let router = routes::router()
        .with(AddData::new(redis.clone()))
        .with(AddData::new(pg.clone()))
        .with(AddData::new(build_schema()));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(router)
        .await
}