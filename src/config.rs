use local_ip_address::local_ip;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Net {
    pub ip: String,
    pub port1: u16,
    pub port2: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DB {
    pub folder: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub net: Net,
    pub database: DB,
}

pub fn load_config() -> Config {
    let file_string = fs::read_to_string("config.json").unwrap();
    let mut obj: Config = serde_json::from_str(file_string.as_str()).unwrap();
    if obj.net.ip == "local" {
        obj.net.ip = local_ip().unwrap().to_string();
    } else if obj.net.ip == "self" {
        obj.net.ip = "127.0.0.1".to_string();
    }
    obj
}
