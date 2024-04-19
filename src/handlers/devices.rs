pub mod inputs;
pub mod sensors;
use super::errors::APIError;
use super::security::{validate_request, Permissions};
use crate::database::devices_db;

use actix_web::{get, web, HttpRequest, Responder, Result};

#[get("/devices")]
/// Returns a list of json objects representing
/// Devices
pub async fn devices(req: HttpRequest) -> Result<impl Responder> {
    // Check if permissions are correct
    if !validate_request(req, Permissions::Read) {return Err(APIError::InvalidPermission.into())}
    
    // Fetching data
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
pub async fn get_device(id: web::Path<String>, req: HttpRequest) -> Result<impl Responder> {
    // Check if permissions are correct
    if !validate_request(req, Permissions::Read) {return Err(APIError::InvalidPermission.into())}

    // Getting args
    let id = id.into_inner();

    // Fetching data
    match devices_db::services::get_device(id) {
        Ok(object) => Ok(web::Json(object)),
        Err(e) => {
            println!("There was an error {}", e);
            Err(APIError::NotFound.into())
        }
    }
}
