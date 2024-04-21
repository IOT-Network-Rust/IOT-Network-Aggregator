#![allow(unused)]
mod aggregator;
mod api;
mod config;
mod database;
use std::process;
use std::thread;

fn main() {
    // Loading conig
    let conf = config::load_config();

    // Defining API addr
    let api_addr = format!("{}:{}", conf.net.ip.clone(), conf.net.api_port);

    // Defining IOT addr
    let iot_addr = format!("{}:{}", conf.net.ip, conf.net.iot_port);

    // Staring API server
    thread::spawn(move || api::start(api_addr));

    // Starting IOT Server
    let mut iot_server =
        aggregator::server::IotServer::open(iot_addr).expect("Server Failed To Start");
    iot_server.listen();

    // On exit
    process::exit(0);
}
