use actix_web::{App, HttpRequest, HttpServer, Result};

use crate::handlers;
use actix_web::web;

/// Function that adds routes to api services
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(handlers::devices::devices)
            .service(handlers::devices::get_device)
            .service(handlers::devices::inputs::get_device_inputs)
            .service(handlers::devices::sensors::get_device_sensors)
            .service(handlers::devices::sensors::get_device_sensor_data),
    );
}

#[actix_web::main]
/// Start API server
/// This server is responsible for hosting data from
/// connected iot devices onto the web
/// Expect data that is received to be in JSON format
pub async fn start(addr: String) -> std::io::Result<()> {
    println!("Server Running On http://{}/", addr);

    // Starting server
    HttpServer::new(|| App::new().configure(configure_routes))
        .bind(addr)?
        .run()
        .await
}
