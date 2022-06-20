use async_graphql::{Context, Object, Result};

use crate::{models::todo::Todo, AppContext};

#[derive(Default)]
pub struct TodosQuery {}

#[Object]
impl TodosQuery {
  async fn todos<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Todo>> {
    let pool = ctx.data_unchecked::<AppContext>().pool.clone();

    let todos = sqlx::query_as!(Todo, "select id, title, done, user_id from Todos")
      .fetch_all(&pool)
      .await?;

    Ok(todos)
  }

  async fn todo<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> Result<Option<Todo>> {
    let pool = ctx.data_unchecked::<AppContext>().pool.clone();

    let todo = sqlx::query_as!(
      Todo,
      "SELECT id, title, done, user_id FROM Todos WHERE id=$1",
      id
    )
    .fetch_optional(&pool)
    .await?;

    Ok(todo)
  }
}
