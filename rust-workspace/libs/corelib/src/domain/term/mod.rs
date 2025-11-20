pub mod value_objects;


use crate::domain::{
    shared::value_object::{Deleted, Timestamp},
    taxonomy::value_objects::TaxonomyId,
    term::value_objects::TermId,
};


#[derive(Debug, PartialEq, Eq)]
pub struct Term {
    pub id: TermId,
    pub taxonomy_id: TaxonomyId,
    pub parent_id: Option<TermId>,

    pub name: String,
    pub visible: bool,
    pub description: Option<String>,

    pub timestamps: Timestamp,
    pub deleted: Deleted,
}
