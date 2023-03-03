use actix_web::{web, HttpResponse};

pub async fn users(info: web::Path<(u32,)>) -> HttpResponse {
    let id = info.0;
    HttpResponse::Ok().body(format!("Informações do usuário com o ID {}: ...", id))
}