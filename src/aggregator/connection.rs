use super::messages::{Message, ProfileMSG};
use crate::database::{
    device_database,
    device_dbs::{self, Table},
    devices_db::{self, DeviceData},
};
use std::io::{Error, ErrorKind, Read, Result};
use std::net::TcpStream;

/// This represents a connection between a device and the server.
/// It contains data on the device and also stores the messaging stream
pub struct DeviceConnection {
    profile: ProfileMSG,
    stream: TcpStream,
}
impl DeviceConnection {
    /// This function takes an input stream and then it uses that stream
    /// to gain data on the device to then cache.
    pub fn new(mut stream: TcpStream) -> Result<Self> {
        // Getting profile message that always comes with first stream
        let request = Self::read(&mut stream)?; // Throws error if unable to read
        let connect_msg: Message = Message::parse(&request.as_str()).expect("Invalid request");

        // Checking if message is of correct type
        if let Message::PROFILE(profile) = connect_msg {
            // Gathering tables from message
            let mut tables: Vec<device_database::SensorTable> = vec![];
            for sensor in &profile.sensors {
                tables.push(device_database::SensorTable::new(
                    &sensor.label,
                    &sensor.data_type,
                ));
            }

            // Adding device to device_catalog db
            devices_db::add_device(DeviceData::new(&profile.name, &profile.id));

            // Initializing device db
            device_database::init_database(&profile.id, &tables);

            Ok(DeviceConnection { profile, stream })
        } else {
            Err(Error::new(
                ErrorKind::ConnectionRefused,
                "Device could not connect",
            ))
        }
    }

    /// Reads data from stream
    fn read(stream: &mut TcpStream) -> Result<String> {
        // Creating buffer
        let mut buffer: [u8; 1024] = [0; 1024];

        // Writing stream data to buffer
        stream.read(&mut buffer)?;

        // Decoding bytes into utf-8 string
        let request = String::from_utf8_lossy(&buffer[..]).to_string();

        Ok(request)
    }

    pub fn listen(&mut self) {
        println!("Listening to connected device");

        // creating buffer to read message
        loop {
            let mut buffer: [u8; 1024] = [0; 1024];
            if let Ok(n) = self.stream.read(&mut buffer) {
                if n == 0 {
                    continue;
                } // Don't except empty messages

                println!("Received a message");
                let request = String::from_utf8_lossy(&buffer[..]).to_string();
                if let Some(request_message) = Message::parse(&request) {
                    self.handle_request(request_message);
                } else {
                    println!("Failed to parse message: {}", request);
                }
            } else {
                println!("Error reading from stream");
                break; // Exit the loop on error
            }
        }
    }

    /// Handles requests from device
    pub fn handle_request(&mut self, request: Message) {
        // Matching request to request type
        match request {
            Message::UPDATE(entries) => {
                println!("{:?}", entries);

                // Iterate over update requests and insert them in db
                let conn = device_database::open_connection(&String::from("dbs"), &self.profile.id)
                    .unwrap();

                for entry in entries.entries {
                    device_database::insert_value(
                        &conn,
                        &entry.table,
                        &device_database::DataEntry::new(entry.data),
                    )
                    .unwrap();
                }
                conn.close().unwrap();
            }
            Message::PING => {
                println!("Yes im here")
            }
            _ => {
                println!("Message type was not expected");
                println!("TYPE:{:?} DATA:{:?}", request.as_ref(), request.to_string());
            }
        };
    }
}
