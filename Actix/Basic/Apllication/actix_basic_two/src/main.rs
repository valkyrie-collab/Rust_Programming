use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("Rust says hello to server")
}

#[post("/echo")]
async fn echo_hello() -> impl Responder {
    HttpResponse::Ok().body("echo function says hello")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Manual says hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_hello)
            .service(echo_hello).service(
                web::scope("/app")
                    .route("/hey", web::get().to(manual_hello))                
            )  
    }).bind(("127.0.0.7", 8080))?.run().await
}
