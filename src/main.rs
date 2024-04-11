#![allow(unused)]
mod config;
mod database_handler;
mod server;
mod messages;
mod database_file_server;
mod device_catalog;
use std::process;
use std::thread;

fn main() {
    let conf = config::load_config();
    let ip_addr = conf.net.ip.clone();
    let port = conf.net.port;
    thread::spawn(move || {database_file_server::start_server(&ip_addr, port)});
    let mut iot_server =
        server::IotServer::open(&conf.net.ip, port).expect("Failed to start server");
    iot_server.start();

    process::exit(0);
}
