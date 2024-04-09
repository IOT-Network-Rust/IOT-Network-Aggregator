use std::io::{self, Read, Write};

use std::net::{TcpListener, TcpStream};
use std::os::windows::io::AsRawSocket;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use serde_derive::{Deserialize, Serialize};

use crate::devices;

use crate::message::{self, Message};

pub struct IotServer {
    ip: String,
    port: u32,
    listener: TcpListener,
    handles: Vec<thread::JoinHandle<()>>,
}
impl IotServer {
    pub fn open(ip: &str, port: u32) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("{}:{}", &ip, &port))?;

        Ok(IotServer {
            ip: ip.to_string(),
            port,
            listener,
            handles: vec![],
        })
    }

    pub fn start(&mut self) {
        println!(
            "Starting IOT Server On IP:{} PORT:{}...",
            self.ip, self.port
        );
        self.listen();
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Err(_) => {}
                Ok(ok_stream) => {
                    let handle = thread::spawn(move || {
                        let mut device =  DeviceConnection::new( ok_stream).unwrap();
                        device.listen();
                    });
                    self.handles.push(handle);
                }
            }
        }
    }

    pub fn handle_request(mut stream: TcpStream) {
        let mut buffer = [0; 1024]; // Buffer size
        stream
            .read(&mut buffer)
            .expect("Failed To Read From Client"); // Reads stream data then puts it into the buffer

        // Turns buffer data into string but handles messy data
        let request = String::from_utf8_lossy(&buffer[..]).to_string();
        let message = message::Message::parse(&request).expect("Could Not Process Message");

        println!("Request Type {}", message.as_ref());
        println!("Request Data {}", message.get_data());

        let response = "Hello, Client".as_bytes();
        stream.write(response).expect("Failed To Respond");
    }

    pub fn shutdown(&mut self) {
        println!("Shutting Down Server...");
        for handle in self.handles.drain(..) {
            handle.join().unwrap();
        }
        println!("Server Shut Down");
    }
}

struct DeviceConnection {
    device: devices::IotDevice,
    stream: TcpStream,
}
impl DeviceConnection {
    pub fn new(mut stream: TcpStream) -> Result<Self, io::Error> {
        let mut buffer = [0; 1024];
        //stream.write("CONNECT".as_bytes());
        stream.read(&mut buffer).expect("Could Not Read Data");

        let request = String::from_utf8_lossy(&buffer[..]).to_string();
        let connect_msg = message::Message::parse(&request.as_str()).unwrap();
        println!("{:?}", connect_msg);
        match connect_msg {
            message::Message::CONNECT(data) => {
                let device = devices::IotDevice::new(&data);
                Ok(DeviceConnection {
                    device,
                    stream,
                })
            },
            _ => {
                Err(io::Error::new(io::ErrorKind::Other, "LOL"))
            }
        }
        
    }

    pub fn listen(&mut self) {
        loop {
            let mut buffer = [0; 1024]; // Buffer size

            // Waiting for Request from device
            match self.stream.read(&mut buffer) {
                Ok(n) => {
                    if n > 0 {
                        let request = String::from_utf8_lossy(&buffer[..]).to_string();
                        self.handle_request(request);
                    } else {
                        continue;
                    }
                }
                Err(_) => {
                    continue;
                }
            };
        }
    }

    pub fn handle_request(&mut self, request: String) {
        let msg = message::Message::parse(&request.as_str()).unwrap();
        println!("TYPE:{:?} DATA:{:?}", msg.as_ref(), msg.get_data());
    }
}
