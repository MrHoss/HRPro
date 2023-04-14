use mysql::{Pool, prelude::Queryable, params, Params};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}

impl User {
    pub fn from_row(row: mysql::Row,login:bool) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name").unwrap(),
            email: row.get("email").unwrap(),
            password: if login { row.get("password") } else { None },
        }
    }
}

pub fn find_all_users(db_pool: &Pool) -> Result<Vec<User>, mysql::Error> {
    let mut conn = db_pool.get_conn()?;
    let query:&str = "SELECT id, name, email FROM users";
    let results: Vec<(Option<String>, String, String)> = conn.query_map(query, |(id, name, email)| (id, name, email))?;
    Ok(results.iter().map(|(id, name, email)| User { id: id.clone(), name: name.clone(), email: email.clone(), password: None }).collect())
}


pub fn find_user_by_id(db_pool: &Pool, user_id: u32,login:bool) -> Option<User> {
    let mut conn = db_pool.get_conn().unwrap();
    let query = format!("SELECT id,name,email,password FROM users WHERE id = {}", user_id);
    let result: Option<mysql::Row> = conn.query_first(query).unwrap();
    match result {
        Some(row) =>Some(User::from_row(row,login)),
        None => None,
    }
}

pub fn register_user(db_pool: &Pool, user: &User) -> Result<(), mysql::Error> {
    let mut conn = db_pool.get_conn()?;
    let password_hash = match &user.password {
        Some(password) => Some(hash(password, DEFAULT_COST).unwrap()),
        None => None,
    };
    let query:&str = "INSERT INTO users (name, email, password) VALUES (:name, :email, :password)";
    let params: Params = params! {
        "name" => &user.name,
        "email" => &user.email,
        "password" => &password_hash,
    };
    conn.exec_drop(query, params)?;
    Ok(())
}

pub fn update_user(db_pool: &Pool, user: &User) -> Result<(), mysql::Error> {
    let mut conn = db_pool.get_conn()?;
    let password_hash = match &user.password {
        Some(password) => Some(hash(password, DEFAULT_COST).unwrap()),
        None => None,
    };
    let query:&str = "UPDATE users SET name = :name, email = :email, password = :password WHERE id = :id";
    let params:Params = params! {
        "id" => &user.id,
        "name" => &user.name,
        "email" => &user.email,
        "password" => &password_hash,
    };
    conn.exec_drop(query, params)?;
    Ok(())
}

pub fn delete_user(db_pool: &Pool, user_id: u32) -> Result<(), mysql::Error> {
    let mut conn = db_pool.get_conn()?;
    let query:&str = "DELETE FROM users WHERE id = :id";
    let params:Params = params! {
        "id" => user_id,
    };
    conn.exec_drop(query, params)?;
    Ok(())
}
