#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    compass::serve().await.unwrap();
}
