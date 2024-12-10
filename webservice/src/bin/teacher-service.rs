use std::io;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer};
use crate::routers::{course_routes, general_routes};
use crate::state::AppState;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../models.rs"]
mod models;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "i'm ok".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}