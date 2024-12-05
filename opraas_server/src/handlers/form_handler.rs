use crate::models::form_data::FormData;
use crate::utils::zip::create_zip;
use actix_web::http::header::ContentDisposition;
use actix_web::{web, HttpResponse, Result};

pub async fn handle_form(form: web::Form<FormData>) -> Result<HttpResponse> {
    // Generate the ZIP file
    let zip_data = create_zip(&form.name, &form.email, &form.message)?;

    // Create a response with the ZIP file
    Ok(HttpResponse::Ok()
        .content_type("application/zip")
        .insert_header(ContentDisposition::attachment("form_data.zip"))
        .body(zip_data))
}
