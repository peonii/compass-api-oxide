use std::env;

use poem::{Server, listener::TcpListener};

pub mod schema;
pub mod routes;
pub mod database;
pub mod librus;

pub async fn serve() -> Result<(), std::io::Error> {
    let pg = database::connect().await.unwrap();
    let redis = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();

    let router = routes::router(pg, redis);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(router)
        .await
}