use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use std::convert::From;
use uuid::Error as ParseError;
use sqlx::Error as SqlError;

#[derive(Debug, Display)]
pub enum ServiceError {

    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "No data found")]
    NoData,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError().body("Internal Server Error. Try again later."),
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().body(message)
            },
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().body("Unauthorized.")
            },
            ServiceError::NoData => {
                HttpResponse::NotFound().body("No data found")
            }
        }
    }
}

impl From<Box<dyn std::error::Error>> for ServiceError {
    fn from(_: Box<dyn std::error::Error>) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<SqlError> for ServiceError {
  fn from(_: SqlError) -> ServiceError {
    ServiceError::InternalServerError
  }
}
