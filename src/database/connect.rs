use mysql::{Pool};

pub fn establish_connection(database_url: &str) -> Pool {
    let pool:Pool = mysql::Pool::new(database_url).expect("Erro não foi possível conectar-se ao banco de dados");
    pool
}
