use std::collections::HashMap;

use crate::{
    domain::{
        shared::{error::DomainError, specs::Specification},
        user::{ports::respository::UserRepository, value_object::UserId},
    },
    predule::{User, UserService},
};

pub struct InMemoryUserRepository {
    by_id: HashMap<UserId, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&mut self, user: &User) -> Result<(), DomainError> {
        self.by_id.insert(user.id.clone(), user.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        Ok(self.by_id.get(id).cloned())
    }

    async fn query(
        &self,
        spec: &(dyn Specification<User> + Send + Sync),
        limit: usize,
        offset: usize,
    ) -> Result<Vec<User>, DomainError> {
        let mut filtered: Vec<&User> = self
            .by_id
            .values()
            .filter(|u| spec.is_satisfied_by(u))
            .collect();

        // Deterministic ordering
        filtered.sort_by(|a, b| {
            a.timestamps
                .created_at()
                .cmp(&b.timestamps.created_at())
                .then_with(|| a.id.cmp(&b.id))
        });

        Ok(filtered
            .into_iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect())
    }

    async fn delete(&mut self, id: &UserId) -> Result<(), DomainError> {
        // If needed you can return error when user does not exist
        // For now, soft-ignore it.
        self.by_id.remove(id);
        Ok(())
    }
}

pub type IUserserviceInMemoryUserRepository = UserService<InMemoryUserRepository>;
