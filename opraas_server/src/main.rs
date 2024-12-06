mod handlers;
mod routes;
mod utils;

use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let router = routes::configure().layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
