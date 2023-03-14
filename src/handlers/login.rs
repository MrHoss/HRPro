use actix_web::{web,HttpResponse,Responder};
use tera::{Tera,Context};
use crate::languages::pt_br::{pt_br};


pub async fn login(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "Home");
    context.insert("translations", &pt_br());

    let rendered = tera.render("login.tera", &context).unwrap();

    HttpResponse::Ok().body(rendered)
}
