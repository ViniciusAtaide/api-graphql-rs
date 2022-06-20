use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use sqlx::FromRow;

use crate::AppContext;

use super::todo::Todo;

#[derive(Debug, FromRow, SimpleObject, Clone, Default)]
#[graphql(complex)]
pub struct User {
  pub id: i32,
  pub username: String,
}

#[ComplexObject]
impl User {
  async fn todos<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Todo>> {
    let pool = ctx.data_unchecked::<AppContext>().pool.clone();

    let todos = sqlx::query_as!(
      Todo,
      "SELECT id, title, done, user_id from Todos WHERE user_id=$1",
      self.id
    )
    .fetch_all(&pool)
    .await?;

    Ok(todos)
  }
}
