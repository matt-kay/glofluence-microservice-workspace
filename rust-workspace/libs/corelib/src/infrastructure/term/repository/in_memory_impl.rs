use std::collections::HashMap;

use crate::domain::{
    shared::{error::DomainError, specs::Specification},
    term::{Term,ports::respository::TermRepository, value_objects::TermId},
};

use crate::application::term::service::TermService;

pub struct InMemoryTermRepository {
    by_id: HashMap<TermId, Term>,
}

impl InMemoryTermRepository {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl TermRepository for InMemoryTermRepository {
    async fn save(&mut self, user: &Term) -> Result<(), DomainError> {
        self.by_id.insert(user.id.clone(), user.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &TermId) -> Result<Option<Term>, DomainError> {
        Ok(self.by_id.get(id).cloned())
    }

    async fn query(
        &self,
        spec: &(dyn Specification<Term> + Send + Sync),
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Term>, DomainError> {
        let mut filtered: Vec<&Term> = self
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

    async fn delete(&mut self, id: &TermId) -> Result<(), DomainError> {
        // If needed you can return error when user does not exist
        // For now, soft-ignore it.
        self.by_id.remove(id);
        Ok(())
    }
}

pub type ITermserviceInMemoryTermRepository = TermService<InMemoryTermRepository>;
