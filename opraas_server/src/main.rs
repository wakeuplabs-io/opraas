use std::net::SocketAddr;

mod handlers;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);

    let app = routes::configure();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
