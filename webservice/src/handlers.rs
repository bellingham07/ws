use actix_web::{web, HttpResponse};
use chrono::Utc;
use crate::models::Course;
use crate::state::AppState;
use std::{env};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(new_course: web::Json<Course>, app_state: web::Data<AppState>) -> HttpResponse {
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
    // app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("course added")
}

pub async fn get_courses_for_teacher(app_state: web::Data<AppState>, params: web::Path<(usize,)>) -> HttpResponse {
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
    HttpResponse::Ok().json("no course found for this teacher".to_string())
    // }
}

// path 就是路劲参数的个数
pub async fn get_courses_detail(app_state: web::Data<AppState>, params: web::Path<(usize, usize)>) -> HttpResponse {
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
    HttpResponse::Ok().json("no course found for this teacher".to_string())
    // }
}

// 测试了一下，两者都可以
fn generate_uuid() -> usize {
    Utc::now().timestamp().try_into().unwrap()
    // Utc::now().timestamp_millis().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::{env, io};
    use actix_web::App;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use super::*;

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

        let course = web::Json(Course {
            teacher_id: 1,
            name: "test course".into(),
            id: None,
            time: None,
        });
        // let app_state = web::Data::new(AppState {
        //     health_check_response: "".to_string(),
        //     visit_count: Mutex::new(0),
        //     courses: Mutex::new(vec![]),
        // });
        let resp = new_course(course, app_state).await;
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

        let teacher_id = web::Path::from((1,));

        let resp = get_courses_for_teacher(app_state, teacher_id).await;

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

        let resp = get_courses_detail(app_state, params).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    fn test_generate_uuid() {
        println!("generate uuid:{}", generate_uuid());
    }
}