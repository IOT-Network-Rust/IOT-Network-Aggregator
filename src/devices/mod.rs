#![allow(unused)]

mod inputs;
mod sensors;

use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::database;

/// Represents an IoT Device.
pub struct IotDevice {
    name: String,
    id: u32,
    database: database::IotDataBase,
}

impl IotDevice {
    
}
