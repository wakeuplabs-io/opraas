mod handlers;
mod utils;

use std::sync::Arc;
use crate::handlers::{build_handler, health_handler};
use axum::routing::{get, post};
use axum::{Extension, Router};
use lambda_http::{run, Error};
use opraas_core::{
    application::CreateProjectService,
    infra::{
        project::{GitVersionControl, InMemoryProjectRepository},
        stack::repo_inmemory::GitStackInfraRepository,
    },
};
use tower_http::trace::{self, TraceLayer};
use tracing::{level_filters::LevelFilter, Level};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let create_service = Arc::new(CreateProjectService::new(
        Box::new(InMemoryProjectRepository::new()),
        Box::new(GitVersionControl::new()),
        Box::new(GitStackInfraRepository::new()),
    ));

    let router = Router::new()
        .route("/health", get(health_handler))
        .route("/build", post(build_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(Extension(create_service));

    run(router).await
}
