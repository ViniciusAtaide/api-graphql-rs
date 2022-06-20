pub enum MyError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}
impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PoolError(ref Error) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }

            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
