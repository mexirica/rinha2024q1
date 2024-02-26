use actix_web::{App, get, HttpServer};
use actix_web::web::Json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

 // Controllers
