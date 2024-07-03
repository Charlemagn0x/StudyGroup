use actix_web::{web, HttpResponse};
use crate::AppState;

async fn get_db_pool_status(data: web::Data<AppState>) -> HttpResponse {
    let state = data.get_ref();
    let conn = state.pool.get();
    match conn {
        Ok(_) => HttpResponse::Ok().json("Database connection is OK"),
        Err(_) => HttpResponse::Ok().json("Database connection is Not OK"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/status").route(web::get().to(get_db_pool_status)));
}
```

```rust
#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod models;
mod routes;
mod schema;

pub struct AppState {
    pub pool: r2d2::Pool<ConnectionManager<SqliteConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: r2d2::Pool<ConnectionManager<SqliteConnection>> =
        r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                pool: pool.clone(),
            })
            .configure(routes::init_zones) // This now includes the server status define
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}