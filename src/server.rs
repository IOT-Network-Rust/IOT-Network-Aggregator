use std::io::{self, Read, Write};

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::devices::IotDevice;

use super::devices;


pub fn listen(ip:String, port:usize) -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", &ip, &port))?;

    let mut handles = vec![];

    let mut iot_devices:Vec<IotDevice> = vec![];


    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprint!("{}", e);
            },
            Ok(mut ok_stream) => {
                let handle = thread::spawn(move || {
                   handle_request(ok_stream);
                });
                handles.push(handle);
            }
        }
    }

    // Joining threads to the main thread
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

pub fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Buffer size
    stream
        .read(&mut buffer)
        .expect("Failed To Read From Client"); // Reads stream data then puts it into the buffer

    // Turns buffer data into string but handles messy data
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received Request");

    let response = "Hello, Client".as_bytes();
    stream.write(response).expect("Failed To Respond");
}


