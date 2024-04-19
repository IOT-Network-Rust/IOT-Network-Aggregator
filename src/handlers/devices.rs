pub mod inputs;
pub mod sensors;
use super::errors::APIError;
use crate::database::devices_db;

use actix_web::{get, web, Responder, Result};

#[get("/devices")]
/// Returns a list of json objects representing
/// Devices
pub async fn devices() -> Result<impl Responder> {
    // Matching result
    match devices_db::services::get_all_devices() {
        Ok(object) => Ok(web::Json(object)),
        Err(e) => {
            println!("There was an error {}", e);
            Err(APIError::InternalError.into())
        }
    }
}

#[get("/devices/{id}")]
/// Returns object about a single device
pub async fn get_device(id: web::Path<String>) -> Result<impl Responder> {
    // Getting args
    let id = id.into_inner();

    // Matching result
    match devices_db::services::get_device(id) {
        Ok(object) => Ok(web::Json(object)),
        Err(e) => {
            println!("There was an error {}", e);
            Err(APIError::NotFound.into())
        }
    }
}
