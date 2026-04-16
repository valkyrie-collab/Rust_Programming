use actix_web::{post, error, web, App, HttpServer, HttpResponse, Responder};
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
        format!("Welcome: To the game user: {} message: {}", path_data.user_id, path_data.message)
    )
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        let config_json_size: web::JsonConfig = web::JsonConfig::default()
            .limit(5000)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });

        App::new().service(
            web::scope("/api")
                .app_data(config_json_size)
                .service(index)
        )
    })
    .bind(("127.0.0.7", 8080))?
    .run()
    .await
}
