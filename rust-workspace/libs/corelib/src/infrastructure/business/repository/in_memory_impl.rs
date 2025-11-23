use std::collections::HashMap;

use crate::domain::{
    business::{Business, ports::respository::BusinessRepository, value_objects::BusinessId},
    shared::{error::DomainError, specs::Specification},
};

use crate::application::business::service::BusinessService;

pub struct InMemoryBusinessRepository {
    by_id: HashMap<BusinessId, Business>,
}

impl InMemoryBusinessRepository {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl BusinessRepository for InMemoryBusinessRepository {
    async fn save(&mut self, user: &Business) -> Result<(), DomainError> {
        self.by_id.insert(user.id.clone(), user.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &BusinessId) -> Result<Option<Business>, DomainError> {
        Ok(self.by_id.get(id).cloned())
    }

    async fn query(
        &self,
        spec: &(dyn Specification<Business> + Send + Sync),
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Business>, DomainError> {
        let mut filtered: Vec<&Business> = self
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

    async fn delete(&mut self, id: &BusinessId) -> Result<(), DomainError> {
        // If needed you can return error when user does not exist
        // For now, soft-ignore it.
        self.by_id.remove(id);
        Ok(())
    }
}

pub type IBusinessserviceInMemoryBusinessRepository = BusinessService<InMemoryBusinessRepository>;
