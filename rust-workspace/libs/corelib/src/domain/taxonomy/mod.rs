use crate::domain::{
    shared::value_object::{Deleted, Timestamp},
    taxonomy::value_objects::TaxonomyId,
};

pub mod value_objects;

#[derive(Debug, PartialEq, Eq)]
pub struct Taxonomy {
    pub id: TaxonomyId,
    pub parent_id: Option<TaxonomyId>,

    pub name: String,
    pub visible: bool,
    pub description: Option<String>,


    pub timestamps: Timestamp,
    pub deleted: Deleted,
}
