use crate::domain::{
    business::{Business, value_objects::BusinessId},
    shared::{error::DomainError, specs::Specification},
};
use async_trait::async_trait;

#[async_trait]
pub trait BusinessRepository: Send + Sync {
    async fn save(&mut self, user: &Business) -> Result<(), DomainError>;

    async fn find_by_id(&self, id: &BusinessId) -> Result<Option<Business>, DomainError>;

    async fn query(
        &self,
        spec: &(dyn Specification<Business> + Send + Sync),
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Business>, DomainError>;

    async fn delete(&mut self, id: &BusinessId) -> Result<(), DomainError>;
}
