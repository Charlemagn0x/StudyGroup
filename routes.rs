use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

mod models;
use models::{StudyGroup, Participant, Meeting, create_study_group, add_participant, schedule_meeting};

async fn create_group(group: web::Json<StudyGroup>) -> impl Responder {
    match create_study_group(&group).await {
        Ok(_) => HttpResponse::Ok().json("Group created successfully"),
        Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(format!("Error creating group: {:?}", e)), // Assume your error supports Debug trait
    }
}

async fn add_group_participant(path: web::Path<(i32, i32)>) -> impl Responder {
    let (group_id, participant_id) = path.into_inner();
    match add_participant(group_id, participant_id).await {
        Ok(_) => HttpResponse::Ok().json("Participant added successfully"),
        Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(format!("Error adding participant: {:?}", e)), // Change accordingly
    }
}

async fn schedule_group_meeting(data: web::Json<Meeting>) -> impl Responder {
    match schedule_meeting(&data).await {
        Ok(_) => HttpResponse::Ok().json("Meeting scheduled successfully"),
        Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(format!("Error scheduling meeting: {:?}", e)), // Adapt as needed
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASEUTO_URL must be set in .env file");

    HttpServer::new(|| {
        App::new()
            .route("/create_group", web::post().to(create_group))
            .route("/add_group_participant/{group_id}/{participant_id}", web::post().to(add_group_participant))
            .route("/schedule_meeting", web::post().to(schedule_group_meeting))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}