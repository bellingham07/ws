use sqlx::PgPool;
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};

pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!("SELECT id, name, picture_url, profile FROM teacher")
        .fetch_all(pool)
        .await?;

    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone().unwrap(),
            picture_url: r.picture_url.clone().unwrap(),
            profile: r.profile.clone().unwrap(),
        })
        .collect();
    match teachers.len() {
        0 => Err(MyError::NotFoundError("No teachers found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_one_teachers_db(pool: &PgPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let rows = sqlx::query!("select * from teacher where id =$1",teacher_id)
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone().unwrap(),
            picture_url: r.picture_url.clone().unwrap(),
            profile: r.profile.clone().unwrap(),
        })
        .map_err(|_err| MyError::NotFoundError("teacher id not found".into()))?;
    Ok(rows)
}


pub async fn post_new_teacher_db(pool: &PgPool, new_teacher: CreateTeacher) -> Result<Teacher, MyError> {
    let rows = sqlx::query!("insert into teacher (name,picture_url,profile) values($1,$2,$3) returning id ,name,picture_url,profile",
        new_teacher.name,new_teacher.picture_url,new_teacher.profile
    )
        .fetch_one(pool)
        .await?;

    Ok(Teacher {
        id: rows.id,
        name: rows.name.unwrap(),
        picture_url: rows.picture_url.unwrap(),
        profile: rows.profile.unwrap(),
    })
}

pub async fn update_teacher_details_db(pool: &PgPool, new_teacher: UpdateTeacher, teacher_id: i32) -> Result<Teacher, MyError> {
    let rows = sqlx::query!("select * from teacher where id =$1",teacher_id)
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone().unwrap(),
            picture_url: r.picture_url.clone().unwrap(),
            profile: r.profile.clone().unwrap(),
        })
        .map_err(|_err| MyError::NotFoundError("teacher id not found".into()))?;

    let tmp = Teacher {
        id: rows.id,
        name: if let Some(name) = new_teacher.name {
            name
        } else {
            new_teacher.name.unwrap()
        },
        picture_url: if let Some(picture_url) = new_teacher.picture_url {
            picture_url
        } else {
            new_teacher.picture_url.unwrap()
        },
        profile: if let Some(profile) = new_teacher.profile {
            profile
        } else {
            new_teacher.profile.unwrap()
        },
    };

    let update_row = sqlx::query!("update teacher set name= $1,picture_url=$2,profile=$3 where id =$4 returning id ,name,picture_url,profile",
        tmp.name,tmp.picture_url,tmp.profile,teacher_id
    )
        .fetch_one(pool)
        .await.map(|r| Teacher {
        id: r.id,
        name: r.name.clone().unwrap(),
        picture_url: r.picture_url.clone().unwrap(),
        profile: r.profile.clone().unwrap(),
    })
        .map_err(|_err| MyError::NotFoundError("teacher id not found".into()))?;
    Ok(update_row)
}

pub async fn delete_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<String, MyError> {
    let row =sqlx::query!("delete from teacher where id=$1", teacher_id)
        .execute(pool)
        .await
        .map_err(|_err| MyError::NotFoundError("teacher id not found".into()))?;

    Ok(format!("Successfully deleted teacher {:?}", teacher_id))

    // let row = sqlx::query!("DELETE FROM teachers WHERE id = $1", teacher_id)
    //     .execute(pool)
    //     .await
    //     .map_err(|_err| MyError::NotFound("Unable to delete teacher".into()))?;
    //
    // Ok(format!("Deleted {:?} record", row))
}