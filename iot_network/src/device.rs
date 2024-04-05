use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};


// Represents an IOT Device
struct IotDevice {
    name:str,
    location:Option<String>, //Where it is located so that you don't lose it
    ip:IPAddr,
    port:u8,

    sensors:Vec<Sensor>,
    inputs:Vec<Input>,
}

// Represents a way to access a sensor
struct Sensor { 
    label:String, // What does the data represent
}

struct Input {
    label:String,
}







