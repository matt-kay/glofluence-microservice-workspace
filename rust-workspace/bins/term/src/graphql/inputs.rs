use async_graphql::InputObject;
use uuid::Uuid;

#[derive(InputObject)]
pub struct CreateTermInput {
    pub taxonomy_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub visible: bool,
    pub description: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateTermInput {
    pub taxonomy_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub visible: Option<bool>,
    pub description: Option<String>,
}
