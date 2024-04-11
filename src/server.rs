use std::io::{self, Read, Write};

use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::database_handler::{self, Table};
use crate::device_catalog::{self, DeviceData};

use crate::messages::{Message, parse_profile, parse_update, ProfileMSG, UpdateMSG};
pub struct IotServer {
    listener: TcpListener,
    handles: Vec<thread::JoinHandle<()>>,
}
impl IotServer {
    pub fn open(ip: &str, port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("{}:{}", &ip, &port))?;

        Ok(IotServer {
            listener,
            handles: vec![],
        })
    }

    pub fn start(&mut self) {
        println!(
            "Starting IOT Server...",
        );
        device_catalog::initialize_database();
        self.listen();
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Err(_) => {}
                Ok(ok_stream) => {
                    let handle = thread::spawn(move || {
                        let mut device =  DeviceConnection::new( ok_stream);
                        device.listen();
                    });
                    self.handles.push(handle);
                }
            }
        }
    }

    pub fn shutdown(&mut self) {
        println!("Shutting Down Server...");
        for handle in self.handles.drain(..) {
            handle.join().unwrap();
        }
        println!("Server Shut Down");
    }

    async fn shutdown_signal() {
        tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    }
}

/// This represents a connection between a device and the server.
/// It contains data on the device and also stores the messaging stream
struct DeviceConnection {
    profile: ProfileMSG,
    stream: TcpStream,
}
impl DeviceConnection {
    /// This function takes an input stream and then it uses that stream
    /// to gain data on the device to then cache.
    pub fn new(mut stream: TcpStream) -> Self {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("Could Not Read Data");

        let request = String::from_utf8_lossy(&buffer[..]).to_string();
        let connect_msg = Message::parse(&request.as_str()).unwrap();

        println!("{:?}", connect_msg);
        match connect_msg {
            Message::PROFILE(profile) => {
                let mut tables:Vec<Table> = vec![];
                for sensor in &profile.sensors {
                    tables.push(Table {
                        name: sensor.label.clone(),
                        data_type: sensor.data_type.clone(),
                    })
                }
                device_catalog::add_device(DeviceData::new(&profile.name, &profile.id));

                database_handler::initialize_database(&profile.id, tables);
                DeviceConnection {
                    profile,
                    stream,
                }
            },
            _ => {
                panic!("Connection Couldn't Be Made")
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
                        self.handle_request(Message::parse(&request).unwrap());
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

    pub fn handle_request(&mut self, request: Message) {
        match request {
            Message::UPDATE(entries) => {
                println!("{:?}", entries);
                for entry in entries.entries {
                    database_handler::insert_into_database(&self.profile.id, entry.table, entry.data);
                }                
            },
            _ => {
                println!("TYPE:{:?} DATA:{:?}", request.as_ref(), request.to_string());
            }
        };
        
    }
}
