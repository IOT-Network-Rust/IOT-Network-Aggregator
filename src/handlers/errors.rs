use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum APIError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "not found")]
    NotFound,
}

impl error::ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
