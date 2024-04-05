use std::net::{IpAddr, Ipv4Addr};

/// Represents a Socket Address, consisting of an IP address and a port number.
/// 
/// # Examples
/// 
/// ```
/// use std::net::{IpAddr, Ipv4Addr};
/// let socket_addr = SocketAddr {
///     ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
///     port: 5000,
/// };
/// ```
struct SocketAddr {
    ip: IpAddr,
    port: u16,
}

/// Represents an IoT Device.
struct IotDevice {
    name: String,
    location: Option<String>, // Where it is located so that you don't lose it
    addr: SocketAddr,
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

    /// Constructs a new `IotDevice`.
    fn new(name: String, location: Option<String>, addr: SocketAddr) -> IotDevice {
        IotDevice {
            name, 
            location,
            addr,
            sensors: Vec::new(),
            inputs: Vec::new(),
        }
    }

    fn send() {

    }

    fn listen() {

    }

    fn accept() {

    }
}
