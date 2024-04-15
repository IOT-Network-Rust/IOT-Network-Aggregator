use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum APIError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "not found")]
    NotFound,

    #[display(fmt = "timeout")]
    Timeout,
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
            APIError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
