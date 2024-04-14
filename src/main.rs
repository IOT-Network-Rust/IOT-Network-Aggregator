#![allow(unused)]
mod aggregator;
mod api;
mod config;
mod database;
mod handlers;
use std::process;
use std::thread;

fn main() {
    let conf = config::load_config();
    let ip_addr = conf.net.ip.clone();
    thread::spawn(move || api::main());
    let mut iot_server = aggregator::server::IotServer::open(&conf.net.ip, conf.net.port2)
        .expect("Failed to start server");
    iot_server.start();

    process::exit(0);
}
