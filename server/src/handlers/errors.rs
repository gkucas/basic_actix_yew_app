use actix_web::{
    error,
    http::{header::ContentType, StatusCode}, HttpResponse,
};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error, Serialize)]
pub enum UserError {
    #[display(fmt = "Validation error: {}", error)]
    ValidationError { error: String },
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}