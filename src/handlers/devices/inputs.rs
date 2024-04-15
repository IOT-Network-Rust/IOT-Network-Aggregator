use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/devices/{id}/inputs")]
/// Returns a list containing device inputs and their
/// Types
pub async fn get_device_inputs(id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    HttpResponse::Ok()
}

#[post("/devices/{id}/inputs/{name}/{value}")]
/// Sets input to certain value
pub async fn set_device_input(
    id: web::Path<String>,
    name: web::Path<String>,
    value: web::Path<String>,
) -> impl Responder {
    HttpResponse::Ok()
}
