use async_graphql::MergedObject;

use super::{todos::TodosQuery, users::UsersQuery};

#[derive(MergedObject, Default)]
pub struct RootQuery(UsersQuery, TodosQuery);
