use local_ip_address;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Net {
    pub ip: String,
    pub api_port: u16,
    pub iot_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DB {
    pub folder: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub net: Net,
    pub database: DB,
}

/// Loads configuration setting for app
/// Uses serde json to properly load data
/// If this fails program Panics
pub fn load_config() -> Config {
    let file_string = fs::read_to_string("config.json").unwrap();
    let mut obj: Config = serde_json::from_str(file_string.as_str()).unwrap();

    // Defines keywords for easy configuration
    // Has capability to use localhost add 127.0.0.1
    // Has capability of using local device ip addr
    if obj.net.ip == "localhost" {
        obj.net.ip = "127.0.0.1".to_string();
    } else if obj.net.ip == "local" {
        obj.net.ip = local_ip_address::local_ip().unwrap().to_string();
    }
    obj
}
