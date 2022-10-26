use std::env;

mod lib;

use dotenv::dotenv;
use lib::run_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
    .await
    .unwrap();
  let server_port = env::var("SERVER_PORT").unwrap().to_owned();
  log::info!(
    "starting HTTP server on port http://localhost:{}",
    server_port
  );
  run_app(server_port, pool).await
}
