use actix_web::{web, HttpResponse};
use chrono::Utc;
use crate::dbaccess::course::{delete_course_db, get_courses_details_db, get_courses_for_teacher_db, post_new_course_db, update_course_details_db};
use crate::state::AppState;
use crate::errors::MyError;
use crate::models::course::{CreateCourse, UpdateCourse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
    // println!("received new course");
    // let course_count = app_state
    //     .courses
    //     .lock().unwrap().clone().into_iter()
    //     .filter(|course| course.teacher_id == new_course.teacher_id)
    //     .collect::<Vec<Course>>().len();
    //
    // // todo 这里是自增id，后续可以考虑换成uuid或者写个函数自己生成，mad，不等后面了，现在就写一个
    // let new_course = Course {
    //     teacher_id: new_course.teacher_id,
    //     id: Some(course_count + 1),
    //     name: new_course.name.clone(),
    //     time: Some(Utc::now().naive_utc()),
    // };
    // app_state.courses.lock().unwrap().push(new_course); HttpResponse::Ok().json(course)
}

pub async fn get_courses_for_teacher(app_state: web::Data<AppState>, params: web::Path<i32>) -> Result<HttpResponse, MyError> {
    // use sql
    // let teacher_id = i32::try_from(params.0).unwrap();
    let teacher_id = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))

    // let teacher_id = params.0;
    //
    // let filtered_courses = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| course.teacher_id == teacher_id)
    //     .collect::<Vec<Course>>();
    //
    // if filtered_courses.len() > 0 {
    //     HttpResponse::Ok().json(filtered_courses)
    // } else {
    // todo 这里可以考虑使用其它语言封装一层结构体进行返回？
    // HttpResponse::Ok().json(courses)
    // }
}

// path 就是路劲参数的个数
pub async fn get_courses_detail(app_state: web::Data<AppState>, params: web::Path<(i32, i32)>) -> Result<HttpResponse, MyError> {
    // use sql
    // let teacher_id = i32::try_from(params.0).unwrap();
    // let course_id = i32::try_from(params.1).unwrap();
    let (teacher_id, course_id) = params.into_inner();
    get_courses_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))

    // let (teacher_id, course_id) = params.0;
    //
    // let selected_courses = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .find(|x| x.teacher_id == teacher_id && x.id == Some(course_id))
    //     .ok_or("course not found");
    //
    // if let Ok(course) = selected_courses {
    //     HttpResponse::Ok().json(course)
    // } else {
    // todo 这里可以考虑使用其它语言封装一层结构体进行返回？
    // HttpResponse::Ok().json("no course found for this teacher".to_string())
    // }
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

// 测试了一下，两者都可以
fn generate_uuid() -> usize {
    Utc::now().timestamp().try_into().unwrap()
    // Utc::now().timestamp_millis().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::{env};
    use actix_web::{web, ResponseError};
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use crate::handlers::course::{delete_course};
    use crate::models::course::{CreateCourse, UpdateCourse};
    use super::*;

    // 很奇怪，一起跑这个就失败，单独跑就能成功
    // #[ignore] // 忽略测试
    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test course123".into(),
            description: Some("This is a course123".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English123".into()),
            level: Some("Beginner123".into()),
        });
        // let app_state = web::Data::new(AppState {
        //     health_check_response: "".to_string(),
        //     visit_count: Mutex::new(0),
        //     courses: Mutex::new(vec![]),
        // });
        let resp = post_new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let teacher_id: web::Path<i32> = web::Path::from(1);

        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params = web::Path::from((1, 1));

        let resp = get_courses_detail(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_failure() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params = web::Path::from((1, 100));

        let resp = get_courses_detail(app_state, params).await;

        match resp {
            Ok(_) => { println!("something is wrong...") }

            Err(err) => { assert_eq!(err.status_code(), StatusCode::NOT_FOUND) }
        }
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let update_course = UpdateCourse {
            name: Some("Course name changed".into()),
            description: Some("This is another test course".into()),
            format: None,
            level: Some("Intermediate".into()),
            price: None,
            duration: None,
            language: Some("Chinese".into()),
            structure: None,
        };

        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let update_param = web::Json(update_course);
        let resp = update_course_details(
            app_state,
            update_param,
            params,
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = delete_course(
            app_state,
            params,
        ).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_course_failure() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        // let updated_course = UpdateCourse { name: "something changed".to_string() };
        let params = web::Path::from((1, 1010));
        // let update_param = web::Json(updated_course);
        let resp = delete_course(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }


    #[test]
    fn test_generate_uuid() {
        // println!("generate uuid:{}", generate_uuid());
    }
}