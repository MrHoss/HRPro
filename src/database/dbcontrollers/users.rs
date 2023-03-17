use mysql::{Pool, prelude::Queryable, params};
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}

impl User {
    pub fn from_row(row: mysql::Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name").unwrap(),
            email: row.get("email").unwrap(),
            password: None,
        }
    }
}

pub fn find_all_users(db_pool: &Pool) -> Result<Vec<User>, mysql::Error> {
    let mut conn = db_pool.get_conn()?;
    let query = "SELECT id, name, email FROM users";
    let results: Vec<(Option<String>, String, String)> = conn.query_map(query, |(id, name, email)| (id, name, email))?;
    Ok(results.iter().map(|(id, name, email)| User { id: id.clone(), name: name.clone(), email: email.clone(), password: None }).collect())
}


pub fn find_user_by_id(db_pool: &Pool, user_id: u32) -> Option<User> {
    let mut conn = db_pool.get_conn().unwrap();
    let query = format!("SELECT id,name,email FROM users WHERE id = {}", user_id);
    let result: Option<mysql::Row> = conn.query_first(query).unwrap();
    println!("{:?}",result);
    match result {
        Some(row) => Some(User::from_row(row)),
        None => None,
    }
}

pub fn register_user(db_pool: &Pool, user: &User) -> Result<(), mysql::Error> {
    let mut conn = db_pool.get_conn()?;
    let password_hash = match &user.password {
        Some(password) => Some(hash(password, DEFAULT_COST).unwrap()),
        None => None,
    };
    let query = "INSERT INTO users (name, email, password) VALUES (:name, :email, :password)";
    let params = params! {
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
    let query = "UPDATE users SET name = :name, email = :email, password = :password WHERE id = :id";
    let params = params! {
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
    let query = "DELETE FROM users WHERE id = :id";
    let params = params! {
        "id" => user_id,
    };
    conn.exec_drop(query, params)?;
    Ok(())
}
