use crate::handlers::{build_handler, health_handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/build").route(web::post().to(build_handler::build_form)))
        .service(web::resource("/health").route(web::get().to(health_handler::health)));
}
