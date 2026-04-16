use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("hello server")
}

#[post("/echo")]
async fn echo_hello() -> impl Responder {
    HttpResponse::Ok().body("echo hello from server")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("manual hello to server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(say_hello)
            .service(echo_hello).route("/hey", web::get().to(manual_hello))  
    }).bind(("127.0.0.7", 8080))?.run().await
}
