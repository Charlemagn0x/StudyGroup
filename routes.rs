use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

mod models;
use models::{add_participant, create_study_group, schedule_meeting, Meeting, Participant, StudyGroup};

async fn create_group(group: web::Json<StudyGroup>) -> impl Responder {
    match create_study_group(&group).await {
        Ok(_) => HttpResponse::Ok().json("Group created successfully"),
        Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(format!("Error creating group: {:?}", e)),
    }
}

async fn add_group_participant(path: web::Path<(i32, i32)>) -> impl Responder {
    let (group_id, participant_id) = path.into_inner();
    match add_participant(group_id, participant_id).await {
        Ok(_) => HttpResponse::Ok().json("Participant added successfully"),
        Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(format!("Error adding participant: {:?}", e)),
    }
}

async fn schedule_group_meeting(data: web::Json<Meeting>) -> impl Responder {
    match schedule_meeting(&data).await {
        Ok(_) => HttpResponse::Ok().json("Meeting scheduled successfully"),
        Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(format!("Error scheduling meeting: {:?}", e)),
    }
}

#[activ_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

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