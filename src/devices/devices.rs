#![allow(unused)]

use super::inputs::Input;
use super::sensors::Sensor;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// Represents an IoT Device.
pub struct IotDevice {
    name: String,
    location: String, // Where it is located so that you don't lose it
    listener: TcpListener,

    sensors: Vec<Sensor>,
    inputs: Vec<Input>,
}

impl IotDevice {
    /// Adds a sensor to the device.
    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    /// Adds an input to the device.
    pub fn add_input(&mut self, input: Input) {
        self.inputs.push(input);
    }

    /// Sends through TCP connection
    pub fn send(&self, stream: &mut TcpStream, data: &[u8]) -> std::io::Result<usize> {
        stream.write(data)
    }

    /// Creates IotDevice
    /// if unable to connect throws error
    ///
    pub fn listen(name: String, location: String, address: &str) -> std::io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        Ok(IotDevice {
            name,
            location,
            listener,
            sensors: Vec::new(),
            inputs: Vec::new(),
        })
    }

    pub fn accept(&self) -> std::io::Result<TcpStream> {
        self.listener.accept().map(|(stream, _)| stream)
    }
}
