use async_graphql::Result;
use sqlx::{Pool, Postgres};

use crate::models::user::User;

pub async fn fetch_all(pool: Pool<Postgres>) -> Result<Vec<User>> {
  let users = sqlx::query_as!(User, "select id, username from Users")
    .fetch_all(&pool)
    .await?;

  Ok(users)
}
