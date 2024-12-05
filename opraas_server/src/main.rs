mod handlers;
mod models;
mod routes;
mod utils;

use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("Listening on http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::configure::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
