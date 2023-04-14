use actix_web::{web};
use crate::handlers::auth::*;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/register")
            .route(web::post().to(register_post))
            .route(web::get().to(register_get)))
        .route("/login",web::get().to(login))
        .route("/logout", web::get().to(login));
}
