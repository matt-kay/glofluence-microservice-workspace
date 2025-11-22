use std::collections::HashMap;

use crate::domain::{
    shared::{error::DomainError, specs::Specification},
    taxonomy::{Taxonomy,ports::respository::TaxonomyRepository, value_objects::TaxonomyId},
};

use crate::application::taxonomy::service::TaxonomyService;

pub struct InMemoryTaxonomyRepository {
    by_id: HashMap<TaxonomyId, Taxonomy>,
}

impl InMemoryTaxonomyRepository {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl TaxonomyRepository for InMemoryTaxonomyRepository {
    async fn save(&mut self, user: &Taxonomy) -> Result<(), DomainError> {
        self.by_id.insert(user.id.clone(), user.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &TaxonomyId) -> Result<Option<Taxonomy>, DomainError> {
        Ok(self.by_id.get(id).cloned())
    }

    async fn query(
        &self,
        spec: &(dyn Specification<Taxonomy> + Send + Sync),
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Taxonomy>, DomainError> {
        let mut filtered: Vec<&Taxonomy> = self
            .by_id
            .values()
            .filter(|u| spec.is_satisfied_by(u))
            .collect();

        // Detaxonomyinistic ordering
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

    async fn delete(&mut self, id: &TaxonomyId) -> Result<(), DomainError> {
        // If needed you can return error when user does not exist
        // For now, soft-ignore it.
        self.by_id.remove(id);
        Ok(())
    }
}

pub type ITaxonomyserviceInMemoryTaxonomyRepository = TaxonomyService<InMemoryTaxonomyRepository>;
