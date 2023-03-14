use actix_web::{web::Data, App, HttpServer};
use actix_files::Files;
use tera::Tera;
use crate::routes::*;

mod languages;
mod database;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "mysql://root:@localhost:3306/RH_System";
    let db_pool = database::connect::establish_connection(&database_url);

    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tera.clone()))
            .app_data(Data::new(db_pool.clone()))
            .service(Files::new("static/", "static").show_files_listing())
            .configure(order_routes)
            .configure(auth_routes)
            .configure(treat_routes)
            .configure(user_routes)

    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
