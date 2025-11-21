use async_trait::async_trait;
use crate::domain::{
    shared::error::DomainError,
    user::{User, value_object::UserId},
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;
}
