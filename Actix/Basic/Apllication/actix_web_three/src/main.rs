use actix_web::{get, web, App, HttpServer};
use std::io::Error;

struct AppData {
    name: String
}

#[get("/")]
async fn index(data: web::Data<AppData>) -> String {
    let app_name: &String = &data.name;

    format!("Hello {}", app_name)
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new().app_data(
            web::Data::new(AppData {name: String::from("Rust Server Two")})
        ).service(index)
    }).bind(("127.0.0.7", 8080))?.run().await
}
