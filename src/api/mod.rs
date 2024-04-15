use actix_web::{App, HttpRequest, HttpServer, Result};

use crate::handlers;
use actix_web::web;

fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(handlers::devices::devices)
            .service(handlers::devices::get_device)
            .service(handlers::devices::inputs::get_device_inputs)
            .service(handlers::devices::sensors::get_device_sensors)
            .service(handlers::devices::sensors::get_device_sensor_data), // Add other routes here
    );
}

#[actix_web::main]
pub async fn main(port: u16) -> std::io::Result<()> {
    let address = format!("127.0.0.1:{}", port);
    println!("Server Running On http://{}/", address);
    HttpServer::new(|| App::new().configure(configure_routes))
        .bind(address)?
        .run()
        .await
}
