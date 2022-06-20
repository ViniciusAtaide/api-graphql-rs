use async_graphql::{Context, Object, Result};

use crate::models::todo::Todo;
use crate::models::user::User;
use crate::AppContext;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
  async fn create_todo<'ctx>(
    &self,
    ctx: &Context<'ctx>,
    title: String,
    user_id: i32,
  ) -> Result<Todo> {
    let pool = ctx.data_unchecked::<AppContext>().pool.clone();

    let created_todo = sqlx::query_as!(
      Todo,
      "INSERT INTO Todos(title, user_id) VALUES ( $1, $2 ) RETURNING id, title, done, user_id",
      title,
      user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(created_todo)
  }
  async fn create_user<'ctx>(&self, ctx: &Context<'ctx>, username: String) -> Result<User> {
    let pool = ctx.data_unchecked::<AppContext>().pool.clone();

    let created_user = sqlx::query_as!(
      User,
      "INSERT INTO Users(username) VALUES ($1) RETURNING *",
      username
    )
    .fetch_one(&pool)
    .await?;

    Ok(created_user)
  }
}
