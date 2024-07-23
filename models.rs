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
        meetings (anId) { // Note: The original identifier corrected as per Rust naming conventions.
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

/// Establishes a connection to the SQLite database.
fn establish_connection() -> Result<SqliteConnection, DieselError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect("Error connecting to database")
}

/// Creates a new study group in the database.
fn create_study_group(conn: &SqliteConnection, name: &str, description: Option<&str>) -> Result<usize, DieselError> {
    use self::study_groups::dsl::*;

    let new_group = StudyGroup {
        id: 0, // Note: SQLite auto-increments the ID, this is a placeholder.
        name: name.to_owned(),
        description: description.map(String::from),
    };

    diesel::insert_into(study_groups)
        .values(&new_group)
        .execute(conn)
}

/// Creates a new participant and associates them with a study group.
fn create_participant(conn: &SqliteConnection, group_id: i32, name: &str, email: &str) -> Result<usize, DieselError> {
    use self::participants::dsl::*;

    let new_participant = Participant {
        id: 0, // Placeholder, SQLite auto-increments.
        study_group_id: group_id,
        name: name.to_owned(),
        email: email.to_owned(),
    };

    diesel::insert_into(participants)
        .values(&new_participant)
        .execute(conn)
}

/// Creates a new meeting for a study group.
fn create_meeting(conn: &SqliteConnection, group_id: i32, title: &str, location: &str, time: &str) -> Result<usize, DieselError> {
    use self::meetings::dsl::*;

    let new_meeting = Meeting {
        id: 0, // Placeholder, SQLite auto-increments.
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
    if let Ok(connection) = establish_connection() {
        if let Ok(_) = create_study_group(&connection, "Rust Study Group", Some("Learning Rust together")) {
            println!("Created new study group successfully.");
        } else {
            println!("Failed to create a study group.");
        }

        // Assuming successful creation, the following ID is used just as an example.
        let group_id = 1;

        if let Ok(_) = create_participant(&connection, group_id, "John Doe", "john@example.com") {
            println!("Added new participant successfully.");
        } else {
            println!("Failed to add a new participant.");
        }

        if let Ok(_) = create_meeting(&connection, group_id, "Introduction to Rust", "Library Room 101", "2023-01-01T10:00:00") {
            println!("Scheduled a new meeting successfully.");
        } else {
            println!("Failed to schedule a new meeting.");
        }
    } else {
        println!("Failed to establish connection.");
    }
}