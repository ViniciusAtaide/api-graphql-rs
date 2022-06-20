use std::collections::HashMap;
use std::sync::Arc;

use crate::models::user::User;
use async_graphql::dataloader::*;
use async_graphql::*;
use sqlx::Postgres;

#[derive(Clone)]
pub struct UserLoader {
  pool: sqlx::Pool<Postgres>,
}

impl UserLoader {
  pub fn new(pool: sqlx::Pool<Postgres>) -> Self {
    return Self { pool };
  }
}

#[async_trait::async_trait]
impl Loader<i32> for UserLoader {
  type Value = User;
  type Error = Arc<sqlx::Error>;

  async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
    let ks = keys
      .iter()
      .map(|k| k.to_string())
      .collect::<Vec<String>>()
      .join(",");
    let query = format!("SELECT * FROM Users WHERE id IN ({ks})");
    let users = sqlx::query_as::<_, User>(query.as_str())
      .fetch_all(&self.pool)
      .await?;

    let mut m: HashMap<i32, User> = HashMap::new();

    for u in users {
      m.insert(u.id, u);
    }

    Ok(m)
  }
}
