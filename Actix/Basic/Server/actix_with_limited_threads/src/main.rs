use actix_web::{web, App, HttpResponse, HttpServer};
use std::io::Error;

#[actix_web::main]
async fn main() -> Result<(), Error>{
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
            .route("/test", web::get().to(HttpResponse::Ok))
        )
    })
    .workers(4)
    .bind(("127.0.0.7", 8080))?
    .run()
    .await
}
