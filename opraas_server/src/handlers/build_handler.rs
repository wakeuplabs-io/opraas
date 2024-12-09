use std::env::temp_dir;

use crate::utils::zip::create_zip;
use axum::{response::IntoResponse, Form};
use serde::Deserialize;
use tempfile::NamedTempFile;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
    message: String,
}

pub async fn build_handler(Form(data): Form<FormData>) -> impl IntoResponse {
    let tmp = tempfile::tempdir()?;

    match create_zip(&data.name, &data.email, &data.message) {
        Ok(zip_data) => (
            axum::http::StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "application/zip")],
            zip_data,
        ),
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            [(axum::http::header::CONTENT_TYPE, "text/plain")],
            b"Failed to generate zip file".to_vec(),
        ),
    }
}
