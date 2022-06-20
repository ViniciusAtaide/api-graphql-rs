use async_graphql::{Context, Object, Result};

use crate::{models::user::User, AppContext};

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
  async fn users<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<User>> {
    let pool = ctx.data_unchecked::<AppContext>().pool.clone();

    let users = sqlx::query_as!(User, "select id, username from Users")
      .fetch_all(&pool)
      .await?;

    Ok(users)
  }
}
