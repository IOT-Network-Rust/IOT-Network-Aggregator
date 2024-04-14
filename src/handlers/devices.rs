pub mod inputs;
pub mod sensors;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
pub use web::Path;

#[get("/devices")]
/// Returns a list of json objects representing
/// Devices
pub async fn devices() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/devices/{id}")]
/// Returns object about a single device
pub async fn get_device(id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    HttpResponse::Ok()
}
