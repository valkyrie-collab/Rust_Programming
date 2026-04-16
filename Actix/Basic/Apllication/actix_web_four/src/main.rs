use actix_web::{get, web, App, HttpServer};
use std::sync::{Arc, Mutex, MutexGuard};
use std::io::Error;

struct AppStateWithCounter {
    count: Arc<Mutex<usize>>
}

#[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter: MutexGuard<usize> = data.count.lock().unwrap();
    *counter += 1;

    format!("Counter value is: {}", counter)
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let counter: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    HttpServer::new(move || {
        App::new().app_data(
            web::Data::new(AppStateWithCounter {count: Arc::clone(&counter)})
        ).service(index)
    }).bind(("127.0.0.7", 8080))?.run().await
}
