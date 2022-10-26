use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::{
  dataloader::DataLoader,
  http::{playground_source, GraphQLPlaygroundConfig},
  EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::{Pool, Postgres};

mod loaders;
mod models;
mod resolvers;

use loaders::users::UserLoader;
use resolvers::mutations::root::MutationRoot;
use resolvers::root::RootQuery;

pub type MainSchema = Schema<RootQuery, MutationRoot, EmptySubscription>;
pub struct AppContext {
  pub pool: Pool<Postgres>,
  pub user_loader: DataLoader<UserLoader>,
}

impl AppContext {
  pub fn new(pool: Pool<Postgres>, user_loader: DataLoader<UserLoader>) -> Self {
    return Self { pool, user_loader };
  }
}

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

pub async fn run_app(server_port: String, pool: Pool<Postgres>) -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let user_loader = DataLoader::new(UserLoader::new(pool.clone()), actix_web::rt::spawn);
  let schema = Schema::build(RootQuery::default(), MutationRoot, EmptySubscription)
    .data(AppContext::new(pool, user_loader))
    .finish();
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
