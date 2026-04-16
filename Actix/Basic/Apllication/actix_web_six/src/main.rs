use actix_web::{App, HttpResponse, HttpServer, web};
use std::io::Error;

async fn scope_response() -> String {
    String::from("Scope Config says hello")
}

async fn api_response() -> String {
    String::from("Config says hello")
}

fn scope_config(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/test")
        .route(web::get().to(scope_response))
        .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}

fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/app")
        .route(web::get().to(api_response))
        .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new().configure(scope_config)
        .service(
            web::scope("/api").configure(config)
        ).route("/", web::get().to(|| async {HttpResponse::Ok().body("root")}))
    }).bind(("127.0.0.7", 8080))?.run().await
}
