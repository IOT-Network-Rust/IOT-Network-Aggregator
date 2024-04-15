use super::super::errors::APIError;
use crate::database::{device_dbs, devices_db};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};

#[get("/devices/{id}/sensors")]
/// Returns list of sensors that object has with their
/// Return type
pub async fn get_device_sensors(id: web::Path<String>) -> Result<impl Responder> {
    let sensors = device_dbs::services::get_device_sensors(&id);
    Ok(web::Json(sensors))
}

#[get("/devices/{id}/sensors/{name}/data")]
/// Returns a list containing sensor data logs
pub async fn get_device_sensor_data(
    id: web::Path<String>,
    name: web::Path<String>,
) -> Result<impl Responder> {
    let entries = device_dbs::services::get_device_sensor_data(&id, &name);
    Ok(web::Json(entries))

}

#[get("/devices/{id}/sensors/{name}/data/{range}")]
/// Returns a list containing sensor data logs
/// but only in range from x:y
pub async fn get_device_sensor_data_ranged(
    id: web::Path<String>,
    name: web::Path<String>,
    range: web::Path<String>,
) -> impl Responder {
    let split = range.split(":");

    let id = id.into_inner();
    let name = name.into_inner();
    HttpResponse::Ok()
}
