use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
mod init;
mod security;
mod errors;
mod handlers;

/// Function that adds routes to api services
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(handlers::devices)
            .service(handlers::get_device)
            .service(handlers::inputs::get_device_inputs)
            .service(handlers::sensors::get_device_sensors)
            .service(handlers::sensors::get_device_sensor_data),
    );
}

#[actix_web::main]
/// Start API server
/// This server is responsible for hosting data from
/// connected iot devices onto the web
/// Expect data that is received to be in JSON format
pub async fn start(addr: String) -> std::io::Result<()> {
    println!("Server Running On http://{}/", addr);

    init::init();

    // Starting server
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // Allowing specific origin
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Allowing specific methods
            .allowed_headers(vec!["api_key", "user_id"]) // Allowing specific headers
            .max_age(3600); // Setting max age for preflight requests;
        App::new().wrap(cors).configure(configure_routes)
    })
    .bind(addr)?
    .run()
    .await
}
