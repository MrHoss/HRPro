use actix_web::{web, HttpResponse, Responder};
use tera::{Tera,Context};
use crate::languages::pt_br::pt_br;

pub async fn not_found(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("title","Nada encontrado");
    context.insert("translations", &pt_br());
    let rendered = tera.render("not_found.tera",&context).unwrap();

    HttpResponse::NotFound().body(rendered)
}