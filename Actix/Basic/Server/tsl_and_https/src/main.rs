use actix_web::{web, App, get, HttpServer, HttpRequest, Responder, HttpResponse};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::io::Error;

#[get("/test")]
async fn send_data(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let mut builder: SslAcceptorBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").service(send_data)
        )
    }).bind_openssl("127.0.0.7:8080", builder)?
    .run().await
}

// openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
// -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"