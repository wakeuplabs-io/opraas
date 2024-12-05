use actix_web::{HttpResponse, Responder};

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}
