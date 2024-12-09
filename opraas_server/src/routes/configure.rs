use crate::handlers::{build_handler, health_handler};
use axum::{
    routing::{get, post},
    Router,
};

pub fn configure() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/build", post(build_handler))
}
