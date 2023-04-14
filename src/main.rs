#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

fn main() {
    let database_url = "mysql://root:271208@127.0.0.2:3306/RH_System";
    let conn = MysqlConnection::establish(&database_url).expect("Falha ao estabelecer conexão");

    println!("Conexão estabelecida com sucesso!");
}


