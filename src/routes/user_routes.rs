use actix_web::{web};
use crate::handlers::*;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg
        //.route("/user/{id}", web::get().to(users))
        .route("/users/{id}", web::get().to(get_user_by_id));

}