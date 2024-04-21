use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Result};
use super::super::security::{validate_request, Permissions};
use super::super::errors::APIError;

#[get("/devices/{id}/inputs")]
/// Returns a list containing device inputs and their
/// Types
pub async fn get_device_inputs(id: web::Path<String> ,req: HttpRequest) -> Result<impl Responder> {
    // Check if permissions are correct
    if !validate_request(req, Permissions::Read) {return Err(APIError::InvalidPermission.into())}

    // Getting args
    let id = id.into_inner();

    Ok(HttpResponse::Ok())
}

#[post("/devices/{id}/inputs/{name}/{value}")]
/// Sets input to certain value
pub async fn set_device_input(
    params: web::Path<(String, String, String)>,
    req: HttpRequest,
) -> Result<impl Responder> {
    // Check if permissions are correct
    if !validate_request(req, Permissions::Read) {return Err(APIError::InvalidPermission.into())}

    // Getting args
    let id = &params.0;
    let name = &params.1;
    let value = &params.2;

    Ok(HttpResponse::Ok())
}
