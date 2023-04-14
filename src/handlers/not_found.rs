use actix_web::{web, HttpResponse, Responder};
use tera::{Tera,Context};
use crate::languages::pt_br::pt_br;

pub async fn not_found(tera: web::Data<Tera>) -> Result<HttpResponse, actix_web::Error> {
    let mut context:Context = Context::new();
    context.insert("title","Nada encontrado");
    context.insert("translations", &pt_br());
    let rendered = tera.render("not_found.tera",&context).unwrap();

    Ok(HttpResponse::NotFound().body(rendered))
}