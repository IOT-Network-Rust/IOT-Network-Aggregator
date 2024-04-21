pub mod inputs;
pub mod sensors;
use super::errors::APIError;
use super::security::{validate_request, Permissions, validate_with_token, login_request, validate_token_status};
use crate::database::catalog_database;

use actix_web::{get, web, HttpRequest, Responder, Result};
use serde_json::json;

#[get("/login")]
pub async fn login(req: HttpRequest) -> Result<impl Responder> {
    // Checks for correct login
    match login_request(req) {
        Err(e) => Err(APIError::InternalError.into()),
        Ok(token) => {
            // Returns json containing token
            Ok(web::Json(json!({ "token": token })))

        }
    }
}

#[get("/login/validate")]
pub async fn validate_token(req: HttpRequest) -> Result<impl Responder> {
    // Checks for correct login
    match validate_token_status(req) {
        Err(e) => {
            println!("{}", e);
            Err(APIError::InternalError.into())},
        Ok(status) => {
            // Returns json containing token
            Ok(web::Json(json!({ "status": status })))
        }
    }
}

#[get("/devices")]
/// Returns a list of json objects representing
/// Devices
pub async fn devices(req: HttpRequest) -> Result<impl Responder> {
    // Check if permissions are correct
    if !validate_with_token(req, Permissions::Read).unwrap() {return Err(APIError::InvalidPermission.into())}
    
    // Fetching data
    match catalog_database::get_all_devices() {
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
    match catalog_database::get_device(id) {
        Ok(object) => Ok(web::Json(object)),
        Err(e) => {
            println!("There was an error {}", e);
            Err(APIError::NotFound.into())
        }
    }
}
