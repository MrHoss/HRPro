use actix_web::{web};
use crate::handlers::{users,index};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/users", web::get().to(index::index))
        .route("/users/{id}", web::get().to(users::users));

}