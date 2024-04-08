use std::io::{self, Read, Write};

use std::net::{TcpListener, TcpStream};
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
                Ok(mut ok_stream) => {
                    let handle = thread::spawn(move || {
                        IotServer::handle_request(ok_stream);
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
    pub fn start(stream: TcpStream) {}

    pub fn listen(&mut self) {
        loop {
            let mut buffer = [0; 1024]; // Buffer size

            // Waiting for Request from device
            match self.stream.read(&mut buffer) {
                Ok(_) => {
                    let request = String::from_utf8_lossy(&buffer[..]).to_string();
                    self.handle_request(request);
                }
                Err(_) => {
                    continue;
                }
            };
        }
    }

    pub fn handle_request(&mut self, request: String) {
        let split: Vec<&str> = request.split("|").collect();
        let len = split.get(0);
    }

    pub fn send(&mut self, string: String) {
        let mut buffer = [0; 1024]; // Buffer size

        self.stream.write_all(string.as_bytes());
    }
}
