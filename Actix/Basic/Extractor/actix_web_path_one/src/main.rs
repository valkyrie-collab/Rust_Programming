use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use std::io::Error;

#[get("/users/{user_id}/{friend}")]
async fn index(path_data: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, friend): (u32, String) = path_data.into_inner();

    HttpResponse::Ok().body(format!("Welcome! {} with his friend: {}", user_id, friend))
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").service(index)
        )
    })
    .bind(("127.0.0.7", 8080))?
    .run()
    .await
}
