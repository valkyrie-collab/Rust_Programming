use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;
use std::io::Error;

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    message: String
}


#[get("/user/{user_id}/{message}")]
async fn index(path_data: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome! to rust {}: {}", path_data.user_id, path_data.message))
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").service(index)
        )
    })
    .bind(("127.0.0.7", 8080))?
    .run()
    .await
}
