#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Queryable, Clone, Debug)]
struct User {
    id: i32,
    name: String,
}

static USER_CACHE: Lazy<Mutex<HashMap<i32, User>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Insertable)]
#[table_name="users"]
struct NewUser<'a> {
    name: &'a str,
}

pub fn establish_connection() -> Result<PgConnection, Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set in .env file")?;
    PgConnection::establish(&database_url)
        .map_err(|_| format!("Error connecting to {}", database_url).into())
}

pub fn create_user<'a>(conn: &PgConnection, name: &'a str) -> Result<usize, Box<dyn Error>> {
    use self::schema::users;
    let new_user = NewUser { name };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .map_err(Into::into)
}

pub fn fetch_user_by_id(conn: &PgConnection, user_id: i32) -> Result<User, Box<dyn Error>> {
    let mut cache = USER_CACHE.lock().map_err(|_| "Cache lock error")?;
    if let Some(user) = cache.get(&user_id) {
        println!("Cache hit!");
        return Ok(user.clone());
    }

    let user = users::table.find(user_id).first::<User>(conn)?;
    cache.insert(user_id, user.clone());
    Ok(user)
}

fn main() {
    match establish_connection() {
        Ok(connection) => {
            println!("Successfully connected to the database.");
            match create_user(&connection, "NewUserName") {
                Ok(_) => println!("User created successfully."),
                Err(e) => println!("Error creating user: {}", e),
            }
        },
        Err(e) => println!("Connection error: {}", e),
    }
}