#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

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
        meetings (id) {
            id -> Integer,
            study_group_id -> Integer,
            title -> Text,
            location -> Text,
            time -> Text, // In a real app, consider using a more appropriate type
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

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_study_group(conn: &SqliteConnection, name: &str, description: Option<&str>) -> usize {
    use self::study_groups::dsl::*;

    let new_study_group = StudyGroup {
        id: 0, // Note: Id is auto-incremented, so the actual value here is ignored
        name: name.to_owned(),
        description: description.map(String::from),
    };

    diesel::insert_into(study_groups)
        .values(&new_study_group)
        .execute(conn)
        .expect("Error saving new study group")
}

fn create_participant(conn: &SqliteConnection, group_id: i32, name: &str, email: &str) -> usize {
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
        .expect("Error saving new participant")
}

fn create_meeting(conn: &SqliteConnection, group_id: i32, title: &str, location: &str, time: &str) -> usize {
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
        .expect("Error saving new meeting")
}

fn main() {
    let connection = establish_connection();

    let study_group_id = create_study_group(&connection, "Rust Study Group", Some("Learning Rust together"));
    println!("Created new study group with ID: {}", study_group_id);

    let participant_id = create_participant(&connection, study_group_id as i32, "John Doe", "john@example.com");
    println!("Added new participant with ID: {}", participant_id);

    let meeting_id = create_meeting(&connection, study_group_id as i32, "Introduction to Rust", "Library Room 101", "2023-01-01T10:00:00");
    println!("Scheduled a new meeting with ID: {}", meetingid);
}