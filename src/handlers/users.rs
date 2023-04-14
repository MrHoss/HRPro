use actix_web::{web, HttpResponse,Responder};
use mysql::Pool;
use tera::Context;

use crate::Tera;
use crate::database::*;
use crate::handlers::*;
use crate::languages::pt_br::pt_br;


pub async fn get_user_by_id(db_pool: web::Data<Pool>,tera: web::Data<Tera>, user_id: web::Path<u32>) -> impl Responder {
    match find_user_by_id(&db_pool, user_id.into_inner(),false) {
        Some(user) => {
            let mut context:Context = Context::new();
            context.insert("title", "Home");
            context.insert("translations", &pt_br());
            context.insert("user", &user);
            let rendered = tera.render("user.tera", &context)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            Ok(HttpResponse::Ok().body(rendered))
        }
        None => not_found(tera).await,
    }
} 

pub async fn get_all_users(db_pool: web::Data<Pool>, tera: web::Data<Tera>) -> Result<HttpResponse, actix_web::Error> {
    match find_all_users(&db_pool){
        Ok(users) => {
            let mut context:Context = Context::new();
            context.insert("title", "Home");
            context.insert("translations", &pt_br());
            context.insert("users", &users);
            let rendered = tera.render("users.tera", &context)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            Ok(HttpResponse::Ok().body(rendered))
        }
        Err(_) => not_found(tera).await,
    }
    
}

