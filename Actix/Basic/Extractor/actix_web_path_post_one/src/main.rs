use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::io::Error;

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    message: String
}

#[post("/submit")]
async fn index(path_data: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(
        format!("Welcome: The user with user_id: {} sent a message: {}", path_data.user_id, path_data.message)
    )
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
