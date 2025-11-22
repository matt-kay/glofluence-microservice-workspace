use crate::domain::{
    shared::{error::DomainError, specs::Specification},
    user::{User, value_object::UserId},
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&mut self, user: &User) -> Result<(), DomainError>;

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;

    async fn query(
        &self,
        spec: &(dyn Specification<User> + Send + Sync),
        limit: usize,
        offset: usize,
    ) -> Result<Vec<User>, DomainError>;

    async fn delete(&mut self, id: &UserId) -> Result<(), DomainError> ;
}
