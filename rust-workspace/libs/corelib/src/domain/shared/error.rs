use thiserror::Error;

/// Represents all possible domain errors across the application.
#[derive(Error, Debug)]
pub enum DomainError {
    // =========================
    // Validation / Invariants
    // =========================
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("value object invariant violation: {0}")]
    InvariantViolation(String),

    #[error("entity not found: {0}")]
    EntityNotFound(String),

    #[error("aggregate validation failed: {0}")]
    AggregateValidation(String),

    // =========================
    // Business Rules
    // =========================
    #[error("business rule violated: {0}")]
    BusinessRuleViolation(String),

    #[error("operation not allowed: {0}")]
    OperationNotAllowed(String),

    // =========================
    // Domain Service / Logic
    // =========================
    #[error("domain service error: {0}")]
    DomainServiceError(String),

    #[error("dependency or collaborator failure: {0}")]
    DependencyError(String),

    // =========================
    // Other / Generic
    // =========================
    #[error("unexpected domain error: {0}")]
    Unexpected(String),
}

// Optional: helper constructors
impl DomainError {
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        DomainError::InvalidInput(msg.into())
    }

    pub fn entity_not_found(entity: impl Into<String>) -> Self {
        DomainError::EntityNotFound(entity.into())
    }

    pub fn business_rule_violation(rule: impl Into<String>) -> Self {
        DomainError::BusinessRuleViolation(rule.into())
    }
}
