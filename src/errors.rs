use actix_web::{error::ResponseError, HttpResponse};
use actix_http::error::Error as ActixHttpError;
use actix_web::error::Error as ActixWebError;
use derive_more::Display;
use std::convert::From;
use uuid::Error as ParseError;
use sqlx::Error as SqlError;

#[derive(Debug, Display)]
pub enum ServiceError {

    #[display(fmt = "Internal Server Error: {}", _0)]
    InternalServerError(String),

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
            ServiceError::InternalServerError(ref msg) => HttpResponse::InternalServerError().body(format!("Internal Server Error: {}. Try again later.", msg)),
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
        ServiceError::InternalServerError("Generic Error".to_string())
    }
}

impl From<SqlError> for ServiceError {
  fn from(e: SqlError) -> ServiceError {
    ServiceError::InternalServerError(format!("SQL Error {:?}", e.to_string()))
  }
}

impl From<ActixHttpError> for ServiceError {
  fn from(e: ActixHttpError) -> ServiceError {
    ServiceError::InternalServerError(format!("Webserver error {:?}", e.to_string()))
  }
}

impl From<ActixWebError> for ServiceError {
  fn from(e: ActixWebError) -> ServiceError {
    ServiceError::InternalServerError(format!("Webserver error {:?}", e.to_string()))
  }
}
