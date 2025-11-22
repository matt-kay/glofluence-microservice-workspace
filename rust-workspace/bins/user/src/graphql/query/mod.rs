use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    /// Returns the
    async fn get_user(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
