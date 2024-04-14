pub mod database_file_server;
use actix_web::{HttpRequest, Result};


use actix_web::web;
use crate::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(handlers::devices::devices)
            .service(handlers::devices::get_device)
            .service(handlers::devices::inputs::get_device_inputs)
            .service(handlers::devices::sensors::get_device_sensors)
            .service(handlers::devices::sensors::get_device_sensor_data)
            // Add other routes here
    );
}