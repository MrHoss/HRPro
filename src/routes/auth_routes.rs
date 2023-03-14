use actix_web::{web};
use crate::handlers::{login};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/login", web::get().to(login::login))
        .route("/register", web::get().to(login::login))
        .route("/logout", web::get().to(login::login));
}
