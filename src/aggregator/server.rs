use super::messages::{parse_profile, parse_update, Message, ProfileMSG, UpdateMSG};
use crate::database::{
    device_dbs::{self, Table},
    devices_db::{self, DeviceData},
};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct IotServer {
    listener: TcpListener,
    handles: Vec<thread::JoinHandle<()>>,
    address:String,
}
impl IotServer {
    pub fn open(ip: &str, port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("{}:{}", &ip, &port))?;

        Ok(IotServer {
            listener,
            handles: vec![],
            address: format!("{}:{}", &ip, &port),
        })
    }

    pub fn start(&mut self) {
        println!("Starting IOT Server On {}", self.address);
        devices_db::initialize_database();
        println!("Started");
        self.listen();
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Err(_) => {}
                Ok(ok_stream) => {
                    println!("Received Connection");
                    let handle = thread::spawn(move || {
                        let mut device = DeviceConnection::new(ok_stream);
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
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
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
                let mut tables: Vec<Table> = vec![];
                for sensor in &profile.sensors {
                    tables.push(Table {
                        name: sensor.label.clone(),
                        data_type: sensor.data_type.clone(),
                    })
                }
                devices_db::add_device(DeviceData::new(&profile.name, &profile.id));

                device_dbs::initialize_database(&profile.id, tables);
                DeviceConnection { profile, stream }
            }
            _ => {
                panic!("Connection Couldn't Be Made")
            }
        }
    }

    pub fn listen(&mut self) {
    println!("Listening to connected device");
    
    // creating buffer to read message
    loop {
        let mut buffer: [u8; 1024] = [0; 1024];

        match self.stream.read(&mut buffer) {
            Ok(n) => {
                if n > 0 {
                    println!("Received a message");
                    let request = String::from_utf8_lossy(&buffer[..]).to_string();
                    if let Some(request_message) = Message::parse(&request) {
                        self.handle_request(request_message);
                    } else {
                        println!("Failed to parse message: {}", request);
                    }
                }
            }
            Err(err) => {
                println!("Error reading from stream: {}", err);
                break; // Exit the loop on error
            }
        }
    }
}

    pub fn handle_request(&mut self, request: Message) {
        match request {
            Message::UPDATE(entries) => {
                println!("{:?}", entries);
                for entry in entries.entries {
                    device_dbs::insert_into_database(&self.profile.id, entry.table, entry.data);
                }
            }
            _ => {
                println!("TYPE:{:?} DATA:{:?}", request.as_ref(), request.to_string());
            }
        };
    }
}
