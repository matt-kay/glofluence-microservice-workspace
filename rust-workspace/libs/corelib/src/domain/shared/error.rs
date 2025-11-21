use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("validation: {message}")]
    Validation {
        message: Cow<'static, str>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("conflict: {message}")]
    Conflict {
        message: Cow<'static, str>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("not found: {message}")]
    NotFound {
        message: Cow<'static, str>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("forbidden: {message}")]
    Forbidden {
        message: Cow<'static, str>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl DomainError {
    pub fn validation(msg: impl Into<Cow<'static, str>>) -> Self {
        DomainError::Validation {
            message: msg.into(),
            source: None,
        }
    }

    pub fn conflict(msg: impl Into<Cow<'static, str>>) -> Self {
        DomainError::Conflict {
            message: msg.into(),
            source: None,
        }
    }

    pub fn not_found(msg: impl Into<Cow<'static, str>>) -> Self {
        DomainError::NotFound {
            message: msg.into(),
            source: None,
        }
    }

    pub fn forbidden(msg: impl Into<Cow<'static, str>>) -> Self {
        DomainError::Forbidden {
            message: msg.into(),
            source: None,
        }
    }

    pub fn conflict_with<E>(msg: impl Into<Cow<'static, str>>, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        DomainError::Conflict {
            message: msg.into(),
            source: Some(Box::new(err)),
        }
    }
}
