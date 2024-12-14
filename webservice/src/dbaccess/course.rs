use sqlx::PgPool;
use crate::errors::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};

pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"select *
        from course
        where teacher_id = $1"#,
        teacher_id
    )
        .fetch_all(pool)
        .await?;

    // 手动转化，上面使用了as方法，并且实现了sqlx::FromRow特征，所以不需要手动
    // let courses: Vec<Course> = rows.iter()
    //     .map(|r| Course {
    //         id: Some(r.id),
    //         name: r.name.clone(),
    //         teacher_id: r.teacher_id,
    //         time: Some(NaiveDateTime::from(r.time.unwrap())),
    //     })
    //     .collect();
    //
    // match courses.len() {
    //     0 => { Err(MyError::NotFoundError("Course not found".to_string())) }
    //     _ => Ok(courses),
    // }
    Ok(rows)
}

pub async fn get_courses_details_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Result<Course, MyError> {
    let rows = sqlx::query_as!(
        Course,
        r#"select *
        from course
        where teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
        .fetch_optional(pool)
        .await;

    if let Ok(Some(course)) = rows {
        Ok(course)
    } else {
        Err(MyError::NotFoundError("Course not found".to_string()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) -> Result<Course, MyError> {
    let rows = sqlx::query_as!(
        Course,
        r#"insert into course (teacher_id,name,description,format,structure,duration,price,language,level)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        returning id,teacher_id,name,time,description,format,structure,duration,price,language,level"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level,
    )
        .fetch_one(pool)
        .await?;

    Ok(rows)
}

pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let course_row = sqlx::query!(
        "delete from course where teacher_id = $1 and id = $2",
        teacher_id,
        id,
    )
        .execute(pool)
        .await?;

    Ok(format!("delete {:?} record", course_row))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    // 获取当前课程信息
    let current_course = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1 AND id = $2"#,
        teacher_id,
        id,
    )
        .fetch_one(pool)
        .await
        .map_err(|_err| MyError::NotFoundError("Course not found".to_string()))?;

    // 判断是否需要更新 name
    let name: String = update_course.name.unwrap_or(current_course.name);

    // 更新课程信息并返回更新后的记录
    let updated_course = sqlx::query_as!(
        Course,
        r#"
        UPDATE course
        SET name = $1
        WHERE teacher_id = $2 AND id = $3
        RETURNING *
        "#,
        name,
        teacher_id,
        id,
    )
        .fetch_one(pool)
        .await
        .map_err(|_err| MyError::NotFoundError("Failed to update course".to_string()))?;

    Ok(updated_course)
}
