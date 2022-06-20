use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::{
  dataloader::DataLoader,
  http::{playground_source, GraphQLPlaygroundConfig},
  EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use loaders::users::UserLoader;
use sqlx::{postgres::PgPool, Pool, Postgres};
use std::env;

mod loaders;
mod models;
mod resolvers;

use resolvers::mutations::root::MutationRoot;
use resolvers::root::RootQuery;

pub type MainSchema = Schema<RootQuery, MutationRoot, EmptySubscription>;

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<MainSchema>, req: GraphQLRequest) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

#[get("/")]
async fn graphql_playground() -> impl Responder {
  Html(playground_source(
    GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
  ))
}

pub struct AppContext {
  pub pool: Pool<Postgres>,
  pub user_loader: DataLoader<UserLoader>,
}

impl AppContext {
  pub async fn new() -> Self {
    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
      .await
      .unwrap();

    let user_loader = DataLoader::new(UserLoader::new(pool.clone()), actix_web::rt::spawn);

    return Self { pool, user_loader };
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
  let server_port = env::var("SERVER_PORT").unwrap().to_owned();

  let schema = Schema::build(RootQuery::default(), MutationRoot, EmptySubscription)
    .data(AppContext::new().await)
    .finish();

  log::info!("starting HTTP server on port {}", server_port);

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(schema.clone()))
      .service(graphql)
      .service(graphql_playground)
      .wrap(Cors::permissive())
      .wrap(Logger::default())
  })
  .workers(10)
  .bind(("127.0.0.1", server_port.parse::<u16>().unwrap()))?
  .run()
  .await
}