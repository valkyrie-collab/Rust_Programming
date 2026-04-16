use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::io::Error;

#[get("/users/{user_id}/{message}")]
async fn index(path_data: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, message): (u32, String) = path_data.into_inner();

    HttpResponse::Ok().body(format!("Welcome! user {} has sent message {}", user_id, message))
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let mut builder: SslAcceptorBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").service(index)
        )
    })
    .bind_openssl("127.0.0.7:8080", builder)?
    .run()
    .await
}
