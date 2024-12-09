use std::path::PathBuf;
use crate::utils::zip::zip_folder;
use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use opraas_core::{
    application::{CreateProjectService, TCreateProjectService},
    config::CoreConfig,
    infra::{
        project::{GitVersionControl, InMemoryProjectRepository},
        stack::repo_inmemory::GitStackInfraRepository,
    },
};
use serde::Deserialize;
use tempfile::TempDir;

#[derive(Deserialize)]
pub struct Payload {
    name: String,
}

pub async fn build_handler(Json(data): Json<Payload>) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let config = CoreConfig::default();
    let tmp_dir = TempDir::new().unwrap();

    let create_service = CreateProjectService::new(
        Box::new(InMemoryProjectRepository::new()),
        Box::new(GitVersionControl::new()),
        Box::new(GitStackInfraRepository::new()),
    );

    let project = create_service
        .create(&PathBuf::from(tmp_dir.path()), &config, false)
        .unwrap();

    let zip_buffer = zip_folder(&project.root).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/zip"));
    headers.insert(
        "Content-Disposition",
        HeaderValue::from_str(&format!("attachment; filename=\"{}.zip\"", data.name)).unwrap(),
    );

    let _ = tmp_dir.close();

    Ok((StatusCode::OK, headers, zip_buffer))
}
