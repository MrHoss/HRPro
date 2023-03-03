use actix_web::{ web::Data,App, HttpServer};
use actix_files::Files;
use tera::Tera;

//mod database;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*let database_url = "mysql://root:@localhost/RH_System";
    let db_pool = database::connect::establish_connection(&database_url);*/
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move|| {
        App::new()
            .app_data(Data::new(tera.clone()))
            .service(Files::new("static/", "static").show_files_listing())
            .configure(routes::routes)
    })
    .bind("10.0.0.2:8000")?
    .run()
    .await
}