use actix_web::{web,HttpResponse, HttpRequest,Responder};
use mysql::Pool;
use tera::{Tera,Context};
use crate::{languages::pt_br::pt_br, database::{User, register_user}};
use serde::{Deserialize};

use super::not_found;

#[derive(Debug, Deserialize)]
pub struct RegistrationForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn login(tera: web::Data<Tera>) -> impl Responder {
    let mut context:Context = Context::new();
    context.insert("title", "Login");
    context.insert("translations", &pt_br());

    let rendered:String = tera.render("login.tera", &context).unwrap();

    HttpResponse::Ok().body(rendered)
}

pub async fn register_post(
    tera:web::Data<Tera>,
    db_pool: web::Data<Pool>,
    form: web::Form<RegistrationForm>,
) -> impl Responder {
    let user:User = User{
        id:None,
        name: form.username.clone(),
        email: form.email.clone(),
        password: Some(form.password.clone())
    };
    match register_user(&db_pool, &user) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(_) => not_found(tera).await,
    }


}
pub async fn register_get(tera: web::Data<Tera>)->impl Responder{
    let mut context:Context = Context::new();
    context.insert("title", "Register");
    context.insert("translations", &pt_br());

    let rendered: String = tera.render("register.tera", &context).unwrap();
        HttpResponse::Ok()
            .append_header(("Content-Type", "text/html; charset=utf-8"))
            .body(rendered)
}
