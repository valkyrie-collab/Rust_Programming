use actix_web::{web, App, HttpServer, HttpResponse};
use std::io::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
            .route("/test", web::get().to(async || {
                HttpResponse::Ok().body("Hello world")
            }))
        )
    }).bind(("127.0.0.7", 8080))?.run().await
}
