use crate::domain::{
    shared::{error::DomainError, specs::Specification},
    taxonomy::{Taxonomy, TaxonomyId},
};
use async_trait::async_trait;

#[async_trait]
pub trait TaxonomyRepository: Send + Sync {
    async fn save(&mut self, taxonomy: &Taxonomy) -> Result<(), DomainError>;

    async fn find_by_id(&self, id: &TaxonomyId) -> Result<Option<Taxonomy>, DomainError>;

    async fn query(
        &self,
        spec: &(dyn Specification<Taxonomy> + Send + Sync),
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Taxonomy>, DomainError>;

    async fn delete(&mut self, id: &TaxonomyId) -> Result<(), DomainError>;
}
