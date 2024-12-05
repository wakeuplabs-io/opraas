use actix_web::{HttpResponse, Responder};

pub async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}
