use actix_web::{web, HttpResponse};
use crate::dbaccess::teacher::{delete_teacher_db, get_all_teachers_db, get_one_teachers_db, post_new_teacher_db, update_teacher_details_db};
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::state::AppState;

pub async fn get_all_teacher(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db)
        .await.map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(app_state: web::Data<AppState>, params: web::Path<i32>) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_one_teachers_db(&app_state.db, teacher_id)
        .await.map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn post_new_teacher(app_state: web::Data<AppState>, new_teacher: web::Json<CreateTeacher>) -> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.db, CreateTeacher::from(new_teacher))
        .await.map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>, new_teacher: web::Json<UpdateTeacher>, params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    update_teacher_details_db(&app_state.db, UpdateTeacher::from(new_teacher), teacher_id)
        .await.map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher_details(
    app_state: web::Data<AppState>, params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db, teacher_id)
        .await.map(|teacher| HttpResponse::Ok().json(teacher))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;
    use actix_web::web;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use crate::handlers::teachers::{delete_teacher_details, get_all_teacher, get_teacher_details, post_new_teacher};
    use crate::state::AppState;

    #[actix_rt::test]
    async fn test_get_all_teachers() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let resp = get_all_teacher(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_tutor_detail_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params = web::Path::from(1);
        let resp = get_teacher_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn test_post_teacher_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let new_teacher = CreateTeacher {
            // id: 1,
            name: "Test Teacher".into(),
            picture_url: "Test Description".into(),
            profile: "a teacher in machine learning".into(),
        };
        let params = web::Json(new_teacher);
        let resp = post_new_teacher(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn test_delete_teacher_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        // let params = web::Json(new_teacher);
        let params = web::Path::from(1);
        let resp = delete_teacher_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}