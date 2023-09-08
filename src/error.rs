use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use thiserror;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error_message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("internal server error.")]
    InternalError,
    #[error("{0} not found.")]
    NotFound(String),
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error_message: self.to_string(),
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(error_response) // Serialize the error as JSON
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}