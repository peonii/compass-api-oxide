#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    compass_api::serve().await.unwrap();
}
