#![allow(unused)]
mod devices;
use devices::{devices::IotDevice, inputs::Input, sensors::Sensor};

fn main() {
    println!("Hello, world!");

    let device_name = String::from("Solar Camera");
    let device_location = String::from("Outside On The Front Lawn");
    let device = IotDevice::listen(device_name, device_location, "0.0.0.0:8000");
}
