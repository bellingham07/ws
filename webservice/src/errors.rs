use std::fmt::{Display, Formatter};
use actix_web::{error, HttpResponse};
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFoundError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("db error occurred: {:?}", msg);
                "database error".into()
            }
            MyError::ActixError(msg) => {
                println!("Actix error occurred: {:?}", msg);
                "internal server error".into()
            }
            MyError::NotFoundError(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            },
        }
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::Error> for MyError{
    fn from(error: actix_web::Error) -> Self {
        MyError::ActixError(error.to_string())
    }
}

impl From<sqlx::error::Error> for MyError{
    fn from(err: sqlx::error::Error) -> Self {
        MyError::DBError(err.to_string())
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self{
            MyError::DBError(msg) | MyError::ActixError(msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFoundError(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse{
            error_message: self.to_string()
        })
    }
}