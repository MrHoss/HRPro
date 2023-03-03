use actix_web::{web,HttpResponse,Responder};
use tera::{Tera,Context};

pub async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "Home");
    context.insert("header", "Hello World");
    context.insert("content", "Simple page");

    let rendered = tera.render("index.tera", &context).unwrap();

    HttpResponse::Ok().body(rendered)
}
