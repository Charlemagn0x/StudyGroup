#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::error::Error;

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