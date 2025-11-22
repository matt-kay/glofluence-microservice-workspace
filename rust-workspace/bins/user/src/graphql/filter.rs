use async_graphql::InputObject;
use uuid::Uuid;

#[derive(InputObject, Clone)]
pub struct StringFilterInput {
    pub equals: Option<String>,
    pub contains: Option<String>,
    pub starts_with: Option<String>,
    // add more ops as needed
}

#[derive(InputObject, Clone)]
pub struct UuidFilterInput {
    pub equals: Option<Uuid>,
    // pub in_: Option<Vec<Uuid>>,
}

#[derive(InputObject, Clone)]
pub struct UserFilterInput {
    // per-field filter objects (hybrid style)
    pub first_name: Option<StringFilterInput>,
    pub last_name: Option<StringFilterInput>,
    pub country_term_id: Option<UuidFilterInput>,

    // composition
    #[graphql(name = "AND")]
    pub and: Option<Vec<UserFilterInput>>,
    #[graphql(name = "OR")]
    pub or: Option<Vec<UserFilterInput>>,
    pub not: Option<Box<UserFilterInput>>,
} 

macro_rules! gen_string_specs {
    ($field:ident, $accessor:expr) => {
        paste::paste! {
            pub struct [<$field:camel EqualsSpec>](pub String);
            impl Specification<User> for [<$field:camel EqualsSpec>] {
                fn is_satisfied_by(&self, candidate: &User) -> bool {
                    ($accessor)(candidate).eq(&self.0)
                }
            }

            pub struct [<$field:camel ContainsSpec>](pub String);
            impl Specification<User> for [<$field:camel ContainsSpec>] {
                fn is_satisfied_by(&self, candidate: &User) -> bool {
                    ($accessor)(candidate).contains(&self.0)
                }
            }

            pub struct [<$field:camel StartsWithSpec>](pub String);
            impl Specification<User> for [<$field:camel StartsWithSpec>] {
                fn is_satisfied_by(&self, candidate: &User) -> bool {
                    ($accessor)(candidate).starts_with(&self.0)
                }
            }
        }
    };
}
