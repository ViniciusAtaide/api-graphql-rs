use actix_web::{http::header::ContentType, test, web, App};

use api_graphql_rs;

#[actix_web::test]
async fn test_users_fetch() {
  let app =
    test::init_service(App::new().route("/", web::get().to(api_graphql_rs::main::index))).await;

  let req = test::TestRequest::default()
    .insert_header(ContentType::plaintext())
    .to_request();
}
