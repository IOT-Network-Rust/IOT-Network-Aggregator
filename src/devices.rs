#![allow(unused)]

mod inputs;
mod sensors;
mod db;

use std::io::{Read, Write};
use std::io;
use std::net::{TcpListener, TcpStream};

/// Represents an IoT Device.
pub struct IotDevice {
    name: String,
    location: String, // Where it is located so that you don't lose it

    sensors: Vec<sensors::Sensor>,
    inputs: Vec<inputs::Input>,
}
