use crate::utils::zip::zip_folder;
use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use opraas_core::{
    application::{CreateProjectService, TCreateProjectService},
    config::{AccountsConfig, ArtifactsConfig, CoreConfig, NetworkConfig},
};
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use tempfile::TempDir;

#[derive(Deserialize)]
pub struct Payload {
    name: String,
    config: NetworkConfig,
}

pub async fn build_handler(
    Extension(create_service): Extension<Arc<CreateProjectService>>,
    Json(data): Json<Payload>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/zip"));
    headers.insert(
        "Content-Disposition",
        HeaderValue::from_str(&format!("attachment; filename=\"{}.zip\"", data.name)).unwrap(),
    );

    let config = CoreConfig {
        network: data.config.clone(),
        accounts: AccountsConfig::null(),
        artifacts: ArtifactsConfig::null(),
    };

    let tmp_dir = TempDir::new().unwrap(); // automatically clean up on drop
    let project = create_service
        .create(&PathBuf::from(tmp_dir.path()), &config, false)
        .unwrap();

    let zip_buffer =
        zip_folder(&project.root).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to zip project"));

    Ok((StatusCode::OK, headers, zip_buffer))
}
