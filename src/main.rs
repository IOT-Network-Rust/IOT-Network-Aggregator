#![allow(unused)]
mod devices;
mod server;
mod database;

use rusqlite::{Connection};


const IP_ADDR:&str = "127.0.0.1";
const PORT:usize = 8080;

fn main() {
    server::listen(IP_ADDR.to_string(), PORT).expect("There Was A Problem Creating The Server");
}
