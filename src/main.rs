#![allow(unused)]
mod config;
mod database_handler;
mod server;
mod messages;
mod dbs_hoster;
use std::process;
use std::thread;

fn main() {
    let conf = config::load_config();
    thread::spawn(|| {dbs_hoster::start_server()});
    let mut iot_server =
        server::IotServer::open(&conf.net.ip, conf.net.port).expect("Failed to start server");
    iot_server.start();

    process::exit(0);
}
