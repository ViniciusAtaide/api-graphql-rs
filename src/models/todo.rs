use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use super::user::User;
use crate::AppContext;

#[derive(Debug, SimpleObject, Default)]
#[graphql(complex)]
pub struct Todo {
  pub id: i32,
  pub title: String,
  pub done: bool,
  pub user_id: i32,
}

#[ComplexObject]
impl Todo {
  async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<User> {
    let context = ctx.data_unchecked::<AppContext>();

    let user = context
      .clone()
      .user_loader
      .load_one(self.user_id)
      .await?
      .unwrap_or_default();

    Ok(user)
  }
}
