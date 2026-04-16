use actix_web::{post, web, error, App, HttpServer, HttpResponse, Responder};
use std::io::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    id: u32,
    message: String
}

#[post("/submit")]
async fn index(path_data: Option<web::Form<Info>>) -> impl Responder {

    match path_data {
        None => HttpResponse::BadRequest().body(
            format!("You have not send any data")
        ),
        Some(data) => HttpResponse::Ok().body(
            format!("Welcome! To this unique item user: {}, your message is {}", data.id, data.mes)
    )
    }
    
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        let config_json_size: web::JsonConfig = web::JsonConfig::default()
            .limit(5000)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
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
