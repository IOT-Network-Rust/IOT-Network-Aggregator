pub mod inputs;
pub mod sensors;
use crate::database::{database_handler, device_catalog};
use serde_json;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/devices")]
/// Returns a list of json objects representing
/// Devices
pub async fn devices() -> impl Responder {
    let objects = device_catalog::get_all_devices().unwrap();
    let json = serde_json::to_string(&objects).unwrap();
    HttpResponse::Ok().json(json)
}

#[get("/devices/{id}")]
/// Returns object about a single device
pub async fn get_device(id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    let object = device_catalog::get_device(id).unwrap();
    let json = serde_json::to_string(&object).unwrap();
    HttpResponse::Ok().json(json)
}
