use actix_web::{web};
use crate::handlers::{index};

pub fn order_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index::index))
        .route("/new-order", web::get().to(index::index))
        .route("/cash-flow", web::get().to(index::index));
}