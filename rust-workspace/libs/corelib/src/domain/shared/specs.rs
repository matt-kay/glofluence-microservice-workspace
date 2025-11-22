/// Specification
pub trait Specification<T>: Send + Sync + 'static {
    fn is_satisfied_by(&self, candidate: &T) -> bool;

    // DSL: A && B
    fn and<B>(self, other: B) -> AndSpec<Self, B>
    where
        Self: Sized,
        B: Specification<T>,
    {
        AndSpec(self, other)
    }

    // DSL: A || B
    fn or<B>(self, other: B) -> OrSpec<Self, B>
    where
        Self: Sized,
        B: Specification<T>,
    {
        OrSpec(self, other)
    }

    // DSL: !A
    fn not(self) -> NotSpec<Self>
    where
        Self: Sized,
    {
        NotSpec(self)
    }
}

/// AND Specification
pub struct AndSpec<A, B>(pub A, pub B);

impl<T, A, B> Specification<T> for AndSpec<A, B>
where
    A: Specification<T>,
    B: Specification<T>,
{
    fn is_satisfied_by(&self, t: &T) -> bool {
        self.0.is_satisfied_by(t) && self.1.is_satisfied_by(t)
    }
}

/// OR Specification
pub struct OrSpec<A, B>(pub A, pub B);

impl<T, A, B> Specification<T> for OrSpec<A, B>
where
    A: Specification<T>,
    B: Specification<T>,
{
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.0.is_satisfied_by(candidate) || self.1.is_satisfied_by(candidate)
    }
}

/// NOT Specification
pub struct NotSpec<A>(pub A);

impl<T, A> Specification<T> for NotSpec<A>
where
    A: Specification<T>,
{
    fn is_satisfied_by(&self, t: &T) -> bool {
        !self.0.is_satisfied_by(t)
    }
}






//  filter_gen.rs
// Metadata-driven filter/spec generator and runtime
// use async_graphql::InputObject;
// use chrono::{DateTime, Utc};
// use paste::paste;
// use serde::{Deserialize, Serialize};
// use std::fmt::Write as FmtWrite;
// use uuid::Uuid;

// // -----------------------------------------------------------------------------
// // Domain: Specification trait & combinators (same as your existing trait)
// // -----------------------------------------------------------------------------
// pub trait Specification<T>: Send + Sync {
//     fn is_satisfied_by(&self, candidate: &T) -> bool;

//     fn and<B>(self, other: B) -> AndSpec<Self, B>
//     where
//         Self: Sized,
//         B: Specification<T>,
//     {
//         AndSpec(self, other)
//     }

//     fn or<B>(self, other: B) -> OrSpec<Self, B>
//     where
//         Self: Sized,
//         B: Specification<T>,
//     {
//         OrSpec(self, other)
//     }

//     fn not(self) -> NotSpec<Self>
//     where
//         Self: Sized,
//     {
//         NotSpec(self)
//     }
// }

// // Box impl so Box<dyn Spec> can be used
// impl<T> Specification<T> for Box<dyn Specification<T> + Send + Sync> {
//     fn is_satisfied_by(&self, candidate: &T) -> bool {
//         (&**self).is_satisfied_by(candidate)
//     }
// }

// pub struct AndSpec<A, B>(pub A, pub B);
// impl<T, A, B> Specification<T> for AndSpec<A, B>
// where
//     A: Specification<T>,
//     B: Specification<T>,
// {
//     fn is_satisfied_by(&self, t: &T) -> bool {
//         self.0.is_satisfied_by(t) && self.1.is_satisfied_by(t)
//     }
// }

// pub struct OrSpec<A, B>(pub A, pub B);
// impl<T, A, B> Specification<T> for OrSpec<A, B>
// where
//     A: Specification<T>,
//     B: Specification<T>,
// {
//     fn is_satisfied_by(&self, t: &T) -> bool {
//         self.0.is_satisfied_by(t) || self.1.is_satisfied_by(t)
//     }
// }

// pub struct NotSpec<A>(pub A);
// impl<T, A> Specification<T> for NotSpec<A>
// where
//     A: Specification<T>,
// {
//     fn is_satisfied_by(&self, t: &T) -> bool {
//         !self.0.is_satisfied_by(t)
//     }
// }

// // -----------------------------------------------------------------------------
// // FieldType metadata and FieldMeta description
// // -----------------------------------------------------------------------------
// #[derive(Clone, Copy, Debug)]
// pub enum FieldKind {
//     String,
//     Uuid,
//     Int,
//     Bool,
//     DateTime,
//     // Add float, enum, etc. as needed
// }

// pub struct FieldMeta {
//     pub name: &'static str,
//     pub kind: FieldKind,
//     /// For runtime evaluation in tests / in-memory filters we provide an accessor:
//     /// fn(&User) -> FieldValue
//     pub accessor: fn(&crate::domain::user::User) -> FieldValue<'_>,
// }

// #[derive(Clone, Copy)]
// pub enum FieldValue<'a> {
//     Str(&'a str),
//     Uuid(&'a uuid::Uuid),
//     Int(i64),
//     Bool(bool),
//     Dt(&'a DateTime<Utc>),
// }

// // -----------------------------------------------------------------------------
// // GraphQL per-field filter inputs (covering all ops we promised)
// // -----------------------------------------------------------------------------
// #[derive(InputObject, Debug, Clone)]
// pub struct StringFilterInput {
//     pub equals: Option<String>,
//     pub not_equals: Option<String>,
//     pub contains: Option<String>,
//     pub icontains: Option<String>,
//     pub starts_with: Option<String>,
//     pub ends_with: Option<String>,
//     pub in_values: Option<Vec<String>>,
//     pub not_in_values: Option<Vec<String>>,
// }

// #[derive(InputObject, Debug, Clone)]
// pub struct UuidFilterInput {
//     pub equals: Option<Uuid>,
//     pub not_equals: Option<Uuid>,
//     pub in_values: Option<Vec<Uuid>>,
//     pub not_in_values: Option<Vec<Uuid>>,
// }

// #[derive(InputObject, Debug, Clone)]
// pub struct IntFilterInput {
//     pub equals: Option<i64>,
//     pub not_equals: Option<i64>,
//     pub lt: Option<i64>,
//     pub lte: Option<i64>,
//     pub gt: Option<i64>,
//     pub gte: Option<i64>,
//     pub in_values: Option<Vec<i64>>,
//     pub not_in_values: Option<Vec<i64>>,
// }

// #[derive(InputObject, Debug, Clone)]
// pub struct BoolFilterInput {
//     pub equals: Option<bool>,
//     pub not: Option<bool>,
// }

// #[derive(InputObject, Debug, Clone)]
// pub struct DateTimeFilterInput {
//     pub before: Option<DateTime<Utc>>,
//     pub after: Option<DateTime<Utc>>,
//     pub between: Option<(DateTime<Utc>, DateTime<Utc>)>,
// }

// // Generic order_by input
// #[derive(InputObject, Debug, Clone)]
// pub struct OrderByInput {
//     pub field: String,
//     pub asc: Option<bool>,
// }

// // -----------------------------------------------------------------------------
// // Macro to auto generate field-level specs, and the top-level filter struct.
// // Usage will be shown below for `User`.
// // -----------------------------------------------------------------------------
// macro_rules! gen_leaf_specs_and_input {
//     // string field
//     ($entity:ident, $field:ident, String, $access:expr) => {
//         paste! {
//             // GraphQL input pointer type already defined as StringFilterInput
//             // leaf spec types:
//             pub struct [<$field:camel EqualsSpec>](pub String);
//             impl Specification<crate::domain::user::User> for [<$field:camel EqualsSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     ($access)(u).as_str() == self.0
//                 }
//             }
//             pub struct [<$field:camel NotEqualsSpec>](pub String);
//             impl Specification<crate::domain::user::User> for [<$field:camel NotEqualsSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     ($access)(u).as_str() != self.0
//                 }
//             }
//             pub struct [<$field:camel ContainsSpec>](pub String);
//             impl Specification<crate::domain::user::User> for [<$field:camel ContainsSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     ($access)(u).contains(&self.0)
//                 }
//             }
//             pub struct [<$field:camel IContainsSpec>](pub String);
//             impl Specification<crate::domain::user::User> for [<$field:camel IContainsSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     ($access)(u).to_lowercase().contains(&self.0.to_lowercase())
//                 }
//             }
//             pub struct [<$field:camel StartsWithSpec>](pub String);
//             impl Specification<crate::domain::user::User> for [<$field:camel StartsWithSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     ($access)(u).starts_with(&self.0)
//                 }
//             }
//             pub struct [<$field:camel EndsWithSpec>](pub String);
//             impl Specification<crate::domain::user::User> for [<$field:camel EndsWithSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     ($access)(u).ends_with(&self.0)
//                 }
//             }
//             pub struct [<$field:camel InSpec>](pub Vec<String>);
//             impl Specification<crate::domain::user::User> for [<$field:camel InSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     let v = ($access)(u).to_string();
//                     self.0.iter().any(|s| s == &v)
//                 }
//             }
//             pub struct [<$field:camel NotInSpec>](pub Vec<String>);
//             impl Specification<crate::domain::user::User> for [<$field:camel NotInSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     let v = ($access)(u).to_string();
//                     !self.0.iter().any(|s| s == &v)
//                 }
//             }
//         }
//     };

//     // uuid field
//     ($entity:ident, $field:ident, Uuid, $access:expr) => {
//         paste! {
//             pub struct [<$field:camel EqualsSpec>](pub uuid::Uuid);
//             impl Specification<crate::domain::user::User> for [<$field:camel EqualsSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     ($access)(u) == &self.0
//                 }
//             }
//             pub struct [<$field:camel NotEqualsSpec>](pub uuid::Uuid);
//             impl Specification<crate::domain::user::User> for [<$field:camel NotEqualsSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     ($access)(u) != &self.0
//                 }
//             }
//             pub struct [<$field:camel InSpec>](pub Vec<uuid::Uuid>);
//             impl Specification<crate::domain::user::User> for [<$field:camel InSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     self.0.iter().any(|id| id == ($access)(u))
//                 }
//             }
//             pub struct [<$field:camel NotInSpec>](pub Vec<uuid::Uuid>);
//             impl Specification<crate::domain::user::User> for [<$field:camel NotInSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     !self.0.iter().any(|id| id == ($access)(u))
//                 }
//             }
//         }
//     };

//     // int field
//     ($entity:ident, $field:ident, Int, $access:expr) => {
//         paste!{
//             pub struct [<$field:camel EqualsSpec>](pub i64);
//             impl Specification<crate::domain::user::User> for [<$field:camel EqualsSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     ($access)(u) == self.0
//                 }
//             }
//             pub struct [<$field:camel NotEqualsSpec>](pub i64);
//             impl Specification<crate::domain::user::User> for [<$field:camel NotEqualsSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     ($access)(u) != self.0
//                 }
//             }
//             pub struct [<$field:camel LtSpec>](pub i64);
//             impl Specification<crate::domain::user::User> for [<$field:camel LtSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool { ($access)(u) < self.0 }
//             }
//             pub struct [<$field:camel LteSpec>](pub i64);
//             impl Specification<crate::domain::user::User> for [<$field:camel LteSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool { ($access)(u) <= self.0 }
//             }
//             pub struct [<$field:camel GtSpec>](pub i64);
//             impl Specification<crate::domain::user::User> for [<$field:camel GtSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool { ($access)(u) > self.0 }
//             }
//             pub struct [<$field:camel GteSpec>](pub i64);
//             impl Specification<crate::domain::user::User> for [<$field:camel GteSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool { ($access)(u) >= self.0 }
//             }
//             pub struct [<$field:camel InSpec>](pub Vec<i64>);
//             impl Specification<crate::domain::user::User> for [<$field:camel InSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool {
//                     self.0.iter().any(|i| i == &($access)(u))
//                 }
//             }
//         }
//     };

//     // bool field
//     ($entity:ident, $field:ident, Bool, $access:expr) => {
//         paste!{
//             pub struct [<$field:camel EqualsSpec>](pub bool);
//             impl Specification<crate::domain::user::User> for [<$field:camel EqualsSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool { ($access)(u) == self.0 }
//             }
//         }
//     };

//     // datetime field
//     ($entity:ident, $field:ident, DateTime, $access:expr) => {
//         paste!{
//             pub struct [<$field:camel BeforeSpec>](pub DateTime<Utc>);
//             impl Specification<crate::domain::user::User> for [<$field:camel BeforeSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool { ($access)(u) < &self.0 }
//             }
//             pub struct [<$field:camel AfterSpec>](pub DateTime<Utc>);
//             impl Specification<crate::domain::user::User> for [<$field:camel AfterSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool { ($access)(u) > &self.0 }
//             }
//             pub struct [<$field:camel BetweenSpec>](pub DateTime<Utc>, pub DateTime<Utc>);
//             impl Specification<crate::domain::user::User> for [<$field:camel BetweenSpec>] {
//                 fn is_satisfied_by(&self, u: &crate::domain::user::User) -> bool { let v = ($access)(u); v >= &self.0 && v <= &self.1 }
//             }
//         }
//     };
// }

// // -----------------------------------------------------------------------------
// // Helpers: reduce & combine boxed specs
// // -----------------------------------------------------------------------------
// pub fn and_reduce<T: 'static + Send + Sync>(
//     mut specs: Vec<Box<dyn Specification<T> + Send + Sync>>,
// ) -> Option<Box<dyn Specification<T> + Send + Sync>> {
//     if specs.is_empty() {
//         return None;
//     }
//     let first = specs.remove(0);
//     Some(specs.into_iter().fold(first, |acc, next| Box::new(acc.and(next))))
// }

// pub fn or_reduce<T: 'static + Send + Sync>(
//     mut specs: Vec<Box<dyn Specification<T> + Send + Sync>>,
// ) -> Option<Box<dyn Specification<T> + Send + Sync>> {
//     if specs.is_empty() {
//         return None;
//     }
//     let first = specs.remove(0);
//     Some(specs.into_iter().fold(first, |acc, next| Box::new(acc.or(next))))
// }

// // -----------------------------------------------------------------------------
// // SQL translator: turns spec tree into SQL WHERE clause + params
// // This is a simple translator for leaf spec types defined by macros above.
// // For production use: expand to handle SQL injection, types, placeholders, and different backends.
// // -----------------------------------------------------------------------------
// #[derive(Debug)]
// pub struct SqlWhere {
//     pub sql: String,
//     pub params: Vec<String>, // simple string params; adapt by type for real DB drivers
// }

// impl SqlWhere {
//     pub fn empty() -> Self {
//         SqlWhere {
//             sql: "1=1".to_string(),
//             params: vec![],
//         }
//     }
// }

// /// Very small trait to allow converting boxed specs to SQL. Only some specs will support it.
// /// For the demonstration we will pattern-match on type names (using `type_name`) — ugly but workable.
// /// You can replace it with a richer trait implemented by generated spec types if you want.
// pub fn spec_to_sql<T: 'static + Send + Sync>(
//     spec: &dyn Specification<T>,
//     // mapping from entity field name -> db column
//     col_for_field: &dyn Fn(&str) -> String,
// ) -> Option<SqlWhere> {
//     use std::any::type_name;
//     let tn = type_name::<&dyn Specification<T>>();
//     // We cannot easily downcast trait objects without making leaf specs implement a marker trait.
//     // So: this function is a placeholder. In practice you should implement a `ToSql` trait
//     // on the generated leaf spec types. For now we return None to indicate "no SQL translation".
//     let _ = tn;
//     None
// }

// // -----------------------------------------------------------------------------
// // Top-level macro to generate User-specific things from a single metadata list.
// // Example usage provided below.
// // -----------------------------------------------------------------------------
// macro_rules! generate_entity_specs {
//     (
//         $entity:ident,
//         fields {
//             $( $fname:ident : $ftype:ident => $access:expr ),* $(,)?
//         },
//         nested {
//             $( $nname:ident : $nentity:ident => $naccess:expr ),* $(,)?
//         }
//     ) => {
//         paste! {
//             // Generate per-field specs
//             $(
//                 gen_leaf_specs_and_input!($entity, $fname, $ftype, $access);
//             )*

//             // Build top-level GraphQL Filter Input struct
//             #[derive(async_graphql::InputObject, Debug, Clone)]
//             pub struct [<$entity FilterInput>] {
//                 $(
//                     pub $fname: Option<
//                         // map field kind to GraphQL input type
//                         { 
//                             // type hack: choose correct input type by ftpe
//                             // We cannot put expressions in type positions, so use match-like macro branches
//                         }
//                     >,
//                 )*
//                 #[graphql(name="AND")]
//                 pub and: Option<Vec<[<$entity FilterInput>]>>,
//                 #[graphql(name="OR")]
//                 pub or: Option<Vec<[<$entity FilterInput>]>>,
//                 pub not: Option<Box<[<$entity FilterInput>]>>,
//                 pub order_by: Option<Vec<OrderByInput>>,
//                 pub limit: Option<i32>,
//                 pub offset: Option<i32>,
//             }

//             // Because the above cannot expand the right type inline easily, provide manual `type` aliases:
//             $(
//                 paste! {
//                     #[allow(non_camel_case_types)]
//                     pub type [<$fname FilterType>] =
//                         {
//                             // match ftpe
//                         };
//                 }
//             )*

//             // Now implement converter function (GraphQL -> Box<dyn Spec>)
//             pub fn [<$entity:snake _filter_to_spec>](filter: Option<[<$entity FilterInput>]>) -> Box<dyn Specification<crate::domain::user::User> + Send + Sync> {
//                 // AllowAll
//                 struct AllowAll;
//                 impl Specification<crate::domain::user::User> for AllowAll {
//                     fn is_satisfied_by(&self, _: &crate::domain::user::User) -> bool { true }
//                 }
//                 let mut specs: Vec<Box<dyn Specification<crate::domain::user::User> + Send + Sync>> = vec![];
//                 if let Some(f) = filter {
//                     $(
//                         // expand by kind
//                         paste! {
//                             // string fields
//                             if stringify!($ftype) == "String" {
//                                 if let Some(sf) = f.$fname {
//                                     if let Some(eq) = sf.equals {
//                                         specs.push(Box::new([<$fname EqualsSpec>](eq)));
//                                     }
//                                     if let Some(ne) = sf.not_equals {
//                                         specs.push(Box::new([<$fname NotEqualsSpec>](ne)));
//                                     }
//                                     if let Some(c) = sf.contains {
//                                         specs.push(Box::new([<$fname ContainsSpec>](c)));
//                                     }
//                                     if let Some(ic) = sf.icontains {
//                                         specs.push(Box::new([<$fname IContainsSpec>](ic)));
//                                     }
//                                     if let Some(sw) = sf.starts_with {
//                                         specs.push(Box::new([<$fname StartsWithSpec>](sw)));
//                                     }
//                                     if let Some(ew) = sf.ends_with {
//                                         specs.push(Box::new([<$fname EndsWithSpec>](ew)));
//                                     }
//                                     if let Some(vals) = sf.in_values {
//                                         specs.push(Box::new([<$fname InSpec>](vals)));
//                                     }
//                                     if let Some(vals) = sf.not_in_values {
//                                         specs.push(Box::new([<$fname NotInSpec>](vals)));
//                                     }
//                                 }
//                             }
//                             // uuid fields
//                             else if stringify!($ftype) == "Uuid" {
//                                 if let Some(uf) = f.$fname {
//                                     if let Some(eq) = uf.equals {
//                                         specs.push(Box::new([<$fname EqualsSpec>](eq)));
//                                     }
//                                     if let Some(ne) = uf.not_equals {
//                                         specs.push(Box::new([<$fname NotEqualsSpec>](ne)));
//                                     }
//                                     if let Some(vals) = uf.in_values {
//                                         specs.push(Box::new([<$fname InSpec>](vals)));
//                                     }
//                                     if let Some(vals) = uf.not_in_values {
//                                         specs.push(Box::new([<$fname NotInSpec>](vals)));
//                                     }
//                                 }
//                             }
//                             // int
//                             else if stringify!($ftype) == "Int" {
//                                 if let Some(ifl) = f.$fname {
//                                     if let Some(eq) = ifl.equals {
//                                         specs.push(Box::new([<$fname EqualsSpec>](eq)));
//                                     }
//                                     if let Some(ne) = ifl.not_equals {
//                                         specs.push(Box::new([<$fname NotEqualsSpec>](ne)));
//                                     }
//                                     if let Some(lt) = ifl.lt {
//                                         specs.push(Box::new([<$fname LtSpec>](lt)));
//                                     }
//                                     if let Some(lte) = ifl.lte {
//                                         specs.push(Box::new([<$fname LteSpec>](lte)));
//                                     }
//                                     if let Some(gt) = ifl.gt {
//                                         specs.push(Box::new([<$fname GtSpec>](gt)));
//                                     }
//                                     if let Some(gte) = ifl.gte {
//                                         specs.push(Box::new([<$fname GteSpec>](gte)));
//                                     }
//                                     if let Some(vals) = ifl.in_values {
//                                         specs.push(Box::new([<$fname InSpec>](vals)));
//                                     }
//                                 }
//                             }
//                             // bool
//                             else if stringify!($ftype) == "Bool" {
//                                 if let Some(bf) = f.$fname {
//                                     if let Some(eq) = bf.equals {
//                                         specs.push(Box::new([<$fname EqualsSpec>](eq)));
//                                     }
//                                 }
//                             }
//                             // datetime
//                             else if stringify!($ftype) == "DateTime" {
//                                 if let Some(df) = f.$fname {
//                                     if let Some(before) = df.before {
//                                         specs.push(Box::new([<$fname BeforeSpec>](before)));
//                                     }
//                                     if let Some(after) = df.after {
//                                         specs.push(Box::new([<$fname AfterSpec>](after)));
//                                     }
//                                     if let Some((a,b)) = df.between {
//                                         specs.push(Box::new([<$fname BetweenSpec>](a,b)));
//                                     }
//                                 }
//                             }
//                         }
//                     )*

//                     // nested children: AND / OR / NOT
//                     if let Some(and_children) = f.and {
//                         let mut child_specs = vec![];
//                         for child in and_children {
//                             let s = [<$entity:snake _filter_to_spec>](Some(child));
//                             child_specs.push(s);
//                         }
//                         if let Some(combined) = crate::filter_gen::and_reduce(child_specs) {
//                             specs.push(combined);
//                         }
//                     }
//                     if let Some(or_children) = f.or {
//                         let mut child_specs = vec![];
//                         for child in or_children {
//                             let s = [<$entity:snake _filter_to_spec>](Some(child));
//                             child_specs.push(s);
//                         }
//                         if let Some(combined) = crate::filter_gen::or_reduce(child_specs) {
//                             specs.push(combined);
//                         }
//                     }
//                     if let Some(not_child) = f.not {
//                         let s = [<$entity:snake _filter_to_spec>](Some(*not_child));
//                         specs.push(Box::new(s.not()));
//                     }
//                 } // end if let Some(f)

//                 if specs.is_empty() {
//                     Box::new(AllowAll)
//                 } else {
//                     and_reduce(specs).unwrap_or_else(|| Box::new(AllowAll))
//                 }
//             } // fn
//         } // paste
//     };
// }

// // -----------------------------------------------------------------------------
// // Example: use the generator for User and SocialProfile
// // - You must adjust `accessor` closures to your User API.
// // - The `crate::domain::user::User` type is referenced; adapt the path.
// // -----------------------------------------------------------------------------

// // NOTE: For compilation adapt these accessors to the real signatures in your `User` type.
// // Below I assume:
// // - user.first_name().as_str() -> &str
// // - user.last_name().as_str() -> &str
// // - user.country_term_id().as_uuid() -> &Uuid
// // - user.age() -> i64
// // - user.is_active() -> bool
// // - user.created_at() -> &DateTime<Utc>
// // - user.social_profiles() -> Vec<SocialProfile>
// generate_entity_specs!(
//     User,
//     fields {
//         first_name: String => |u: &crate::domain::user::User| u.first_name().as_str(),
//         last_name: String => |u: &crate::domain::user::User| u.last_name().as_str(),
//         country_term_id: Uuid => |u: &crate::domain::user::User| u.country_term_id().as_uuid(),
//         age: Int => |u: &crate::domain::user::User| u.age(),
//         is_active: Bool => |u: &crate::domain::user::User| u.is_active(),
//         created_at: DateTime => |u: &crate::domain::user::User| u.created_at(),
//     },
//     nested {
//         // example for nested relations (not fully expanded)
//         social_profiles: SocialProfile => |u: &crate::domain::user::User| {
//             // placeholder: you'd probably want specialized nested filter handling
//             FieldValue::Str("") // not used directly
//         }
//     }
// );

// // -----------------------------------------------------------------------------
// // Relay-style Connection types (simple)
// // -----------------------------------------------------------------------------
// #[derive(async_graphql::SimpleObject, Debug, Clone)]
// pub struct PageInfo {
//     pub has_next_page: bool,
//     pub has_previous_page: bool,
//     pub start_cursor: Option<String>,
//     pub end_cursor: Option<String>,
// }

// #[derive(async_graphql::SimpleObject, Debug, Clone)]
// pub struct Edge<T: Clone + Send + Sync> {
//     pub cursor: String,
//     pub node: T,
// }

// #[derive(async_graphql::SimpleObject, Debug, Clone)]
// pub struct Connection<T: Clone + Send + Sync> {
//     pub edges: Vec<Edge<T>>,
//     pub page_info: PageInfo,
//     pub total_count: i64,
// }


