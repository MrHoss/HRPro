use actix_web::{web, HttpResponse,Responder};
use mysql::serde_json::json;
use mysql::Pool;
use crate::Tera;
use crate::database::*;
use crate::handlers::*;

pub async fn get_user_by_id(db_pool: web::Data<Pool>,tera: web::Data<Tera>, user_id: web::Path<u32>) -> impl Responder {
    match find_user_by_id(&db_pool, user_id.into_inner()) {
        Some(user) => {
            let value = json!({"id": &user.id, "name": &user.name, "email": &user.email});
            HttpResponse::Ok().json(&value)
        }
        None => not_found(tera).await,
    }
} 