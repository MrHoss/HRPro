use actix_web::{web,HttpResponse,Responder};
use tera::{Tera,Context};
use crate::languages::pt_br::pt_br;

pub async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context:Context = Context::new();
    context.insert("title", "Home");
    context.insert("header", "Hello World");
    context.insert("content", "Simple page");
    context.insert("translations", &pt_br());

    let rendered = tera.render("index.tera", &context).unwrap();

    HttpResponse::Ok().body(rendered)
}
