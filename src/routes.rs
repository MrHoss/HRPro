use actix_web::{web};
use crate::handlers::{index, users};

// Define a função que configura as rotas e retorna o objeto "App"
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index::index))
        .route("/login", web::get().to(index::index))
        .route("/register", web::get().to(index::index))
        .route("/usuario/{id}", web::get().to(users::users))
        .route("/jobs", web::get().to(index::index))
        .route("/candidates", web::get().to(index::index))
        .route("/myprofile", web::get().to(index::index))
        .route("/panel", web::get().to(index::index));
}
