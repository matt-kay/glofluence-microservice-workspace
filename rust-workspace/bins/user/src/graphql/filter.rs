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







#[Object]
impl Query {
    async fn search_users<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        filter: Option<UserFilterInput>,
        limit: Option<i32>,
        offset: Option<i32>,
        order_by: Option<Vec<OrderByInput>>,
    ) -> Result<Vec<UserGql>, async_graphql::Error> {
        let app_state = ctx.data::<AppState>().map_err(|_| async_graphql::Error::new("app state"))?;
        let mut user_service = app_state.user_service.lock().await;

        let spec = user_filter_to_spec(filter);

        // If repo supports SQL translation we could call:
        // let sql_where = spec_to_sql(spec.as_ref(), &|field| column_map(field));
        // repo.query_by_sql(sql_where, order_by, limit, offset).await

        let limit = limit.unwrap_or(50).max(1) as usize;
        let offset = offset.unwrap_or(0).max(0) as usize;

        let users = user_service.repo.query(spec.as_ref(), limit, offset).await
            .map_err(|e| async_graphql::Error::new(format!("repo error: {}", e)))?;

        Ok(users.into_iter().map(UserGql::from).collect())
    }
}
