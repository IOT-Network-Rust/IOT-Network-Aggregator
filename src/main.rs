#![allow(unused)]
mod aggregator;
mod api;
mod config;
mod database;
mod handlers;
use std::process;
use std::thread;

const TCP_PORT: u16 = 8080;
const API_PORT: u16 = 9000;

fn main() {
    let conf = config::load_config();
    thread::spawn(move || api::main(API_PORT));
    let mut iot_server = aggregator::server::IotServer::open(&conf.net.ip, TCP_PORT)
        .expect("Failed to start server");
    iot_server.start();

    process::exit(0);
}
