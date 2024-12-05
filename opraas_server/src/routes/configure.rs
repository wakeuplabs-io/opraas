use crate::handlers::{form_handler, hello_handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/submit").route(web::post().to(form_handler::handle_form)))
        .service(web::resource("/").route(web::get().to(hello_handler::say_hello)));
}
