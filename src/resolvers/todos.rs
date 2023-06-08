use async_graphql::{Context, Object, Result};

use crate::{models::todo::Todo, AppContext};
use crate::repositories::todos;

#[derive(Default)]
pub struct TodosQuery {}

#[Object]
impl TodosQuery {
  async fn todos<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Todo>> {
    let pool = ctx.data_unchecked::<AppContext>().pool.clone();

    todos::fetch_all(pool).await
  }

  async fn todo<'ctx>(&self, ctx: &Context<'ctx>, id: i32) -> Result<Option<Todo>> {
    let pool = ctx.data_unchecked::<AppContext>().pool.clone();

    todos::fetch_one(pool, id).await
  }
}
