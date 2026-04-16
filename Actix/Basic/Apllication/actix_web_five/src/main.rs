use actix_web::{web, App, HttpResponse, HttpServer, guard};
use std::io::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/")
                .guard(guard::Host("www.rust-lang.org"))
                .route("", web::to(|| async {
                    HttpResponse::Ok().body("www")
                })
            )
        ).service(
            web::scope("/")
                .guard(guard::Host("user.rust-lang.org"))
                .route("", web::to(|| async {
                    HttpResponse::Ok().body("user")
                })
            )
        )
    }).bind(("127.0.0.7", 8080))?.run().await
}
