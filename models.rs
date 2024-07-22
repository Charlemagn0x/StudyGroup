#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error as DieselError;
use dotenv::dotenv;
use std::env;
use serde::{Serialize, Deserialize};

mod schema {
    table! {
        study_groups (id) {
            id -> Integer,
            name -> Text,
            description -> Nullable<Text>,
        }
    }

    table! {
        participants (id) {
            id -> Integer,
            study_group_id -> Integer,
            name -> Text,
            email -> Text,
        }
    }

    table! {
        meetings (anId) { // Corrected field name according to Rust naming convention
            id -> Integer,
            study_group_id -> Integer,
            title -> Text,
            location -> Text,
            time -> Text,
        }
    }
}

use self::schema::{meetings, participants, study_groups};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "study_groups"]
pub struct StudyGroup {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "participants"]
pub struct Participant {
    pub id: i32,
    pub study_group_id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "meetings"]
pub struct Meeting {
    pub id: i32,
    pub study_group_id: i32,
    pub title: String,
    pub location: String,
    pub time: String,
}

fn establish_connection() -> Result<SqliteConnection, DieselError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    match SqliteConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(_) => Err(DieselError::NotFound), // Using NotFound to signify connection error
    }
}

fn create_study_group(conn: &SqliteConnection, name: &str, description: Option<&str>) -> Result<usize, DieselError> {
    use self::study_groups::dsl::*;

    let new_study_group = StudyGroup {
        id: 0, // Note: Id is auto-incremented, so the actual value here is ignored
        name: name.to_owned(),
        description: description.map(String::from),
    };

    diesel::insert_into(study_groups)
        .values(&new_study_group)
        .execute(conn)
}

fn create_participant(conn: &SqliteConnection, group_id: i32, name: &str, email: &str) -> Result<usize, Diesel DreamError> {
    use self::participants::dsl::*;

    let new_participant = Participant {
        id: 0,
        study_group_id: group_id,
        name: name.to_owned(),
        email: email.to_owned(),
    };

    diesel::insert_into(participants)
        .values(&new_participant)
        .execute(conn)
}

fn create_meeting(conn: &SqliteConnection, group_id: i32, title: &str, location: &str, time: &str) -> Result<usize, DieselError> {
    use self::meetings::dsl::*;

    let new_meeting = Meeting {
        id: 0,
        study_group_id: group_id,
        title: title.to_owned(),
        location: location.to_owned(),
        time: time.to_owned(),
    };

    diesel::insert_into(meetings)
        .values(&new_meeting)
        .execute(conn)
}

fn main() {
    match establish_connection() {
        Ok(connection) => {
            let study_group_result = create_study_group(&connection, "Rust Study Group", Some("Learning Rust together"));

            match study_group_result {
                Ok(_) => println!("Created new study group successfully."),
                Err(e) => println!("Failed to create a study group. Error: {}", e),
            }

            // Assuming create_study_group successfully returns the ID in a real scenario, 
            // but for the purpose of this example, the group ID is hardcoded to 1.
            let group_id = 1;

            match create_participant(&connection, group_id, "John Doe", "john@example.com") {
                Ok(_) => println!("Added new participant successfully."),
                Err(e) => println!("Failed to add a new participant. Error: {}", e),
            }

            match create_meeting(&connection, group_id, "Introduction to Rust", "Library Room 101", "2023-01-01T10:00:00") {
                Ok(_) => println!("Scheduled a new meeting successfully."),
                Err(e) => println!("Failed to schedule a new meeting. Error: {}", e),
            }
        },
        Err(e) => println!("Failed to establish connection. Error: {}", e),
    }
}