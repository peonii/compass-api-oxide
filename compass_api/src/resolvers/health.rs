use async_graphql::Object;


#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    async fn health(&self) -> &'static str {
        "Please buy obot"
    }
}