use actix_web::{web, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Olá, mundo!")
}

// Define uma função que será usada como manipulador para a rota "/usuario/{id}"
async fn usuario(info: web::Path<(u32,)>) -> HttpResponse {
    let id = info.0;
    HttpResponse::Ok().body(format!("Informações do usuário com o ID {}: ...", id))
}

// Define a função que configura as rotas e retorna o objeto "App"
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index))
        .route("/usuario/{id}", web::get().to(usuario));
}