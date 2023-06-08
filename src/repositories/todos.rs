use async_graphql::Result;
use sqlx::{Pool, Postgres};

use crate::models::todo::Todo;

pub async fn fetch_all(pool: Pool<Postgres>) -> Result<Vec<Todo>> {
  let todos = sqlx::query_as!(Todo, "SELECT id, title, done, user_id FROM Todos")
    .fetch_all(&pool)
    .await?;

  Ok(todos)
}

pub async fn fetch_one(pool: Pool<Postgres>, id: i32) -> Result<Option<Todo>> {
  let todo = sqlx::query_as!(
    Todo,
    "SELECT id, title, done, user_id FROM Todos WHERE id=$1",
    id
  )
  .fetch_optional(&pool)
  .await?;

  Ok(todo)
}
