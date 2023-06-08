use async_graphql::{Context, Object, Result};

use crate::repositories::{users};
use crate::{models::user::User, AppContext};

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
  async fn users<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<User>> {
    let pool = ctx.data_unchecked::<AppContext>().pool.clone();

    users::fetch_all(pool).await
  }
}
