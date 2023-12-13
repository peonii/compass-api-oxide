#[tokio::main]
async fn main() {
    compass::serve().await.unwrap();
}
