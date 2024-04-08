#![allow(unused)]
mod config;
mod database;
mod devices;
mod message;
mod server;

use rusqlite::Connection;

fn main() {
    let conf = config::load_config();
    let mut iot_server =
        server::IotServer::open(&conf.net.ip, conf.net.port).expect("Failed to start server");
    iot_server.start();
}
