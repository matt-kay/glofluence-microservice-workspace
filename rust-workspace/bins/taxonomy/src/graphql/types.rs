use async_graphql::{ID, SimpleObject};
use corelib::predule::Taxonomy as DomainTaxonomy;
use uuid::Uuid;

#[derive(SimpleObject)]
pub struct Taxonomy {
    pub id: ID,

    pub parent_id: Option<Uuid>,
    pub name: String,
    pub visible: bool,
    pub description: Option<String>,

    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
    pub deletetion_status: Option<String>,
    pub version: u64,
}

impl From<DomainTaxonomy> for Taxonomy {
    fn from(value: DomainTaxonomy) -> Self {
        Self {
            id: value.id.as_uuid().into(),
            parent_id: value.parent_id.map(|v| v.as_uuid()),
            name: value.name.as_str().to_string(),
            visible: value.visible,
            description: value.description.map(|v| v.as_str().to_string()),

            created_at: value.timestamps.created_human(),
            updated_at: value.timestamps.updated_human(),
            deleted: value.deleted.is_deleted(),
            deletetion_status: Some(value.deleted.status()),
            version: value.version,
        }
    }
}
