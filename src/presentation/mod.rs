use actix_web::{App, HttpServer};
// mod handler;
// mod request;
// mod response;
pub mod hello;

use crate::presentation::hello as handler;
use crate::infrastructure::database::db_context;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(db_context::DBContext::new())
            .service(handler::hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}