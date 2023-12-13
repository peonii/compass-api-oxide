use poem::{Server, listener::TcpListener};

pub mod schema;
pub mod routes;
pub mod database;

pub async fn serve() -> Result<(), std::io::Error> {
    let router = routes::router();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(router)
        .await
}