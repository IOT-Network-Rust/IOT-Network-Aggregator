pub mod inputs;
pub mod sensors;
use super::errors::APIError;
use crate::database::{database_handler, device_catalog};

use actix_web::{get, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder, Result};

#[get("/devices")]
/// Returns a list of json objects representing
/// Devices
pub async fn devices() -> Result<impl Responder> {
    match device_catalog::get_all_devices() {
        Ok(objects) => Ok(web::Json(objects)),
        Err(e) => Err(APIError::InternalError.into()),
    }
}

#[get("/devices/{id}")]
/// Returns object about a single device
pub async fn get_device(id: web::Path<String>) -> Result<impl Responder> {
    let id = id.into_inner();
    match device_catalog::get_device(id) {
        Ok(object) => Ok(web::Json(object)),
        Err(_) => Err(APIError::NotFound.into()),
    }
}
