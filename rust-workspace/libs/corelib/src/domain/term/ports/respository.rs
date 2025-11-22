use crate::domain::{
    shared::{error::DomainError, specs::Specification},
    term::{Term, TermId},
};
use async_trait::async_trait;

#[async_trait]
pub trait TermRepository: Send + Sync {
    async fn save(&mut self, term: &Term) -> Result<(), DomainError>;

    async fn find_by_id(&self, id: &TermId) -> Result<Option<Term>, DomainError>;

    async fn query(
        &self,
        spec: &(dyn Specification<Term> + Send + Sync),
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Term>, DomainError>;

    async fn delete(&mut self, id: &TermId) -> Result<(), DomainError>;
}
