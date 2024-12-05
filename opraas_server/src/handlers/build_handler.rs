use crate::models::build_data::BuildData;
use crate::utils::zip::create_zip;
use actix_web::http::header::ContentDisposition;
use actix_web::{web, HttpResponse, Result};

pub async fn build_form(form: web::Form<BuildData>) -> Result<HttpResponse> {
    let zip_data = create_zip(&form.name, &form.email, &form.message)?;

    Ok(HttpResponse::Ok()
        .content_type("application/zip")
        .insert_header(ContentDisposition::attachment("form_data.zip"))
        .body(zip_data))
}
