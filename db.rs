#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub fn establish_connection() -> Result<PgConnection, Box<dyn Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set in .env file")?;

    PgConnection::establish(&database_url)
        .map_err(|_| format!("Error connecting to {}", database_url).into())
}

fn main() {
    match establish_connection() {
        Ok(connection) => {
            println!("Successfully connected to the database.");
        },
        Err(e) => println!("Connection error: {}", e)
    }
}