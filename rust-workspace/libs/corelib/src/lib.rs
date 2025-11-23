mod application;
mod domain;
mod infrastructure;

pub mod predule {
    // Shared
    pub use crate::domain::shared::value_object::Demographics;
    pub use crate::domain::shared::value_object::PhoneNumber;
    pub use crate::domain::shared::value_object::PhysicalAddress;
    pub use crate::domain::shared::value_object::SocialMedia;
    pub use crate::domain::shared::value_object::SocialMediaLink;
    pub use crate::domain::shared::value_object::SocialMediaMetadata;
    pub use crate::domain::shared::value_object::SocialMediaPlatform;
    pub use crate::domain::shared::value_object::SocialPlatformName;
    pub use crate::domain::shared::value_object::Tag;
    pub use crate::domain::shared::value_object::WebsiteUrl;

    // User
    pub use crate::domain::user::User;
    pub use crate::domain::user::value_object::FirstName;
    pub use crate::domain::user::value_object::LastName;
    pub use crate::domain::user::value_object::UserId;

    pub use crate::application::user::service::UserService;

    pub use crate::infrastructure::user::event_bus::in_memory_bus::InMemoryUserEventBus;
    pub use crate::infrastructure::user::event_handlers::in_memory_handlers::InMemoryAuditLogHandler;
    pub use crate::infrastructure::user::event_handlers::in_memory_handlers::InMemoryWelcomeEmailHandler;

    pub use crate::infrastructure::user::repository::in_memory_impl::IUserserviceInMemoryUserRepository;
    pub use crate::infrastructure::user::repository::in_memory_impl::InMemoryUserRepository;

    // Business
    pub use crate::domain::business::Business;
    pub use crate::domain::business::value_objects::BusinessDescription;
    pub use crate::domain::business::value_objects::BusinessFeatures;
    pub use crate::domain::business::value_objects::BusinessHourEntry;
    pub use crate::domain::business::value_objects::BusinessId;
    pub use crate::domain::business::value_objects::BusinessName;
    pub use crate::domain::business::value_objects::ContactInfo;
    pub use crate::domain::business::value_objects::ExtraFeatureKey;
    pub use crate::domain::business::value_objects::ExtraFeatureValue;
    pub use crate::domain::business::value_objects::ServiceName;

    pub use crate::application::business::service::BusinessService;

    pub use crate::infrastructure::business::event_bus::in_memory_bus::InMemoryBusinessEventBus;
    pub use crate::infrastructure::business::event_handlers::in_memory_handlers::InMemoryBusinessAuditLogHandler;

    pub use crate::infrastructure::business::repository::in_memory_impl::IBusinessserviceInMemoryBusinessRepository;
    pub use crate::infrastructure::business::repository::in_memory_impl::InMemoryBusinessRepository;

    // Taxonomy
    pub use crate::domain::taxonomy::Taxonomy;
    pub use crate::domain::taxonomy::value_objects::TaxonomyDescription;
    pub use crate::domain::taxonomy::value_objects::TaxonomyId;
    pub use crate::domain::taxonomy::value_objects::TaxonomyName;

    pub use crate::application::taxonomy::service::TaxonomyService;

    pub use crate::infrastructure::taxonomy::event_bus::in_memory_bus::InMemoryTaxonomyEventBus;
    pub use crate::infrastructure::taxonomy::event_handlers::in_memory_handlers::InMemoryTaxonomyAuditLogHandler;

    pub use crate::infrastructure::taxonomy::repository::in_memory_impl::ITaxonomyserviceInMemoryTaxonomyRepository;
    pub use crate::infrastructure::taxonomy::repository::in_memory_impl::InMemoryTaxonomyRepository;

    // Term
    pub use crate::domain::term::Term;
    pub use crate::domain::term::value_objects::TermDescription;
    pub use crate::domain::term::value_objects::TermId;
    pub use crate::domain::term::value_objects::TermName;

    pub use crate::application::term::service::TermService;

    pub use crate::infrastructure::term::event_bus::in_memory_bus::InMemoryTermEventBus;
    pub use crate::infrastructure::term::event_handlers::in_memory_handlers::InMemoryTermAuditLogHandler;

    pub use crate::infrastructure::term::repository::in_memory_impl::ITermserviceInMemoryTermRepository;
    pub use crate::infrastructure::term::repository::in_memory_impl::InMemoryTermRepository;
}
