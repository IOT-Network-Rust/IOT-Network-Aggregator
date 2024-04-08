#![allow(unused)]
mod devices;
mod server;
mod database;
mod parser;

use rusqlite::{Connection};


const IP_ADDR:&str = "127.0.0.1";
const PORT:u32 = 8080;

fn main() {
    let mut iot_server = server::IotServer::open(IP_ADDR, PORT).expect("Failed to start server");
    iot_server.start();
}
