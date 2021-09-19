#[macro_use]
extern crate diesel;

mod presentation;
mod infrastructure;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(infrastructure::DBContext::new())
            .service(presentation::hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
