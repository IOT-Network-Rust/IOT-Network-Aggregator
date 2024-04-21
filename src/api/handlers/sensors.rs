use super::super::errors::APIError;
use crate::database::{devices_db, device_database};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use super::super::security::{validate_request, Permissions};

#[get("/devices/{id}/sensors")]
/// Returns list of sensors that object has with their
/// Return type
pub async fn get_device_sensors(id: web::Path<String>, req: HttpRequest) -> Result<impl Responder> {
    // Check if permissions are correct
    if !validate_request(req, Permissions::Read) {return Err(APIError::InvalidPermission.into())}

    // Fetching data
    let conn = device_database::open_connection(&String::from("dbs"), &id).unwrap();
    match device_database::get_tables(&conn) {
        Ok(objects) => Ok(web::Json(objects)),
        Err(e) => {
            println!("There was an error {}", e);
            Err(APIError::InternalError.into())
        }
    }
}

#[get("/devices/{id}/sensors/{name}/data")]
// problem generating documentation
/// Returns a list containing sensor data logs
pub async fn get_device_sensor_data(param: web::Path<(String, String)>, req: HttpRequest) -> Result<impl Responder> {
    // Check if permissions are correct
    if !validate_request(req, Permissions::Read) {return Err(APIError::InvalidPermission.into())}

    // Getting args
    let id = &param.0;
    let name = &param.1;

    // Fetching data
    let conn = device_database::open_connection(&String::from("dbs"), &id).unwrap();
    match device_database::get_table_entries(&conn, name) {
        Ok(object) => Ok(web::Json(object)),
        Err(e) => {
            println!("There was an error {}", e);
            Err(APIError::InternalError.into())
        }
    }
}

#[get("/devices/{id}/sensors/{name}/data/{range}")]
/// Returns a list containing sensor data logs
/// but only in range from x:y
pub async fn get_device_sensor_data_ranged(
    params: web::Path<(String, String, String)>,
    req: HttpRequest
) -> Result<impl Responder> {
    // Check if permissions are correct
    if !validate_request(req, Permissions::Read) {return Err(APIError::InvalidPermission.into());}

    // Getting args
    let id = &params.0;
    let name = &params.1;
    let range = &params.2;

    
    let split = range.split(":");

    Ok(HttpResponse::Ok())
}
