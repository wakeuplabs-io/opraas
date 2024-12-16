use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use opraas_core::application::{
    contracts::{StackContractsInspectorService, TStackContractsInspectorService},
    stack::{StackInfraInspectorService, TStackInfraInspectorService},
};
use std::{io::Cursor, sync::Arc};

pub async fn inspect_contracts_handler(
    Extension(contracts_inspector): Extension<Arc<StackContractsInspectorService>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, "Could not find a valid ZIP file"))?
    {
        if let Some(filename) = field.file_name() {
            if filename.ends_with(".zip") {
                let data = field
                    .bytes()
                    .await
                    .map_err(|_| (StatusCode::BAD_REQUEST, "Could not find a valid ZIP file"))?;

                let result = contracts_inspector
                    .inspect(Cursor::new(data.to_vec()))
                    .map_err(|_| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Could not find a valid ZIP file",
                        )
                    })?;

                return Ok(Json(result));
            }
        }
    }

    Err((StatusCode::BAD_REQUEST, "Could not find a valid ZIP file"))
}

pub async fn inspect_infra_handler(
    Extension(infra_deployer): Extension<Arc<StackInfraInspectorService>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, "Could not find a valid ZIP file"))?
    {
        if let Some(filename) = field.file_name() {
            if filename.ends_with(".zip") {
                let data = field
                    .bytes()
                    .await
                    .map_err(|_| (StatusCode::BAD_REQUEST, "Could not find a valid ZIP file"))?;
                let result = infra_deployer
                    .inspect(Cursor::new(data.to_vec()))
                    .map_err(|_| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Could not find a valid ZIP file",
                        )
                    })?;

                return Ok(Json(result));
            }
        }
    }

    Err((StatusCode::BAD_REQUEST, "Could not find a valid ZIP file"))
}
