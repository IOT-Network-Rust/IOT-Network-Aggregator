use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::net::{TcpListener, TcpStream};

/// Represents an IoT Device.
struct IotDevice {
    name: String,
    location: String, // Where it is located so that you don't lose it
    listener: TcpListener,

    sensors: Vec<Sensor>,
    inputs: Vec<Input>,
}

/// Represents a sensor.
struct Sensor {
    label: String, // What does the data represent
}

/// Represents a way to send data to the device.
struct Input {
    label: String, // What the data means
}

impl IotDevice {
    /// Adds a sensor to the device.
    fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    /// Adds an input to the device.
    fn add_input(&mut self, input: Input) {
        self.inputs.push(input);
    }

    /// Sends through TCP connection
    fn send(&self, stream: &mut TcpStream, data: &[u8]) -> std::io::Result<usize> {
        stream.write(data)
    }

    /// Creates IotDevice
    /// if unable to connect throws error
    ///
    fn listen(name: String, location: String, address: &str) -> std::io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        Ok(IotDevice {
            name,
            location,
            listener,
            sensors: Vec::new(),
            inputs: Vec::new(),
        })
    }

    fn accept(&self) -> std::io::Result<TcpStream> {
        self.listener.accept().map(|(stream, _)| stream)
    }
}
