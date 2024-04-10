#![allow(unused)]
mod config;
mod database_handler;
mod server;
mod messages;
use std::process;

fn main() {
    let conf = config::load_config();
    let mut iot_server =
        server::IotServer::open(&conf.net.ip, conf.net.port).expect("Failed to start server");
    iot_server.start();

    process::exit(0);
}
