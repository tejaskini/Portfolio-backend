use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum MyError {
    MongoError(mongodb::error::Error),
    NotFound(String),
    AuthError(String),
    InternalError,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::MongoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
            MyError::AuthError(_) => StatusCode::UNAUTHORIZED,
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            message: self.to_string(),
        })
    }
}

// Allow using '?' on MongoDB calls to automatically convert to MyError
impl From<mongodb::error::Error> for MyError {
    fn from(err: mongodb::error::Error) -> Self {
        MyError::MongoError(err)
    }
}