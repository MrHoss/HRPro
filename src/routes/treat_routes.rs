use actix_web::{web};
use crate::handlers::{not_found};

pub fn treat_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .default_service(web::get().to(not_found::not_found));
}