use mysql::{Pool};

pub fn establish_connection(database_url: &str) -> Pool {
    let pool = mysql::Pool::new(database_url).unwrap();
    pool
}
