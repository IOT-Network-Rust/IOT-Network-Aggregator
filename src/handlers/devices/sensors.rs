use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/devices/{id}/sensors")]
/// Returns list of sensors that object has with their
/// Return type
pub async fn get_device_sensors(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/devices/{id}/sensors/{name}/data")]
/// Returns a list containing sensor data logs
pub async fn get_device_sensor_data(
    id: web::Path<String>,
    name: web::Path<String>,
) -> impl Responder {
    let id = id.into_inner();
    let name = name.into_inner();
    HttpResponse::Ok()
}
