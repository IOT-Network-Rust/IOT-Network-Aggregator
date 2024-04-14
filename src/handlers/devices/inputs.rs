use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/devices/{id}/inputs")]
/// Returns a list containing device inputs and their
/// Types
pub async fn get_device_inputs(id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    HttpResponse::Ok()
}
