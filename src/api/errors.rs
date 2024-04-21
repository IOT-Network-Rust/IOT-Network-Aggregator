use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
/// Represents errors that can be expected from 
/// Using this API
pub enum APIError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "not found")]
    NotFound,

    #[display(fmt = "invalid permissions")]
    InvalidPermission,

    #[display(fmt = "incomplete")]
    Incomplete,
}

impl error::ResponseError for APIError {
    /// Generating response HTML from a error
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    /// Matches HTML error codes with the API error types
    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::NotFound => StatusCode::NOT_FOUND,
            APIError::InvalidPermission => StatusCode::FORBIDDEN,
            APIError::Incomplete => StatusCode::NOT_FOUND,
        }
    }
}
