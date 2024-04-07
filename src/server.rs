use std::io::{self, Read, Write};

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use crate::devices;

pub fn listen(ip: String, port: usize) -> io::Result<()> {
    println!("Server Is Starting From {ip}:{port}");
    let listener = TcpListener::bind(format!("{}:{}", &ip, &port))?;
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let mut handles = vec![];
    let mut iot_devices: Vec<devices::IotDevice> = vec![];

    let (tx, rx) = mpsc::channel();

    println!("Enter STOP To Shut Down The Server");
    // Checks for the STOP command to shutdown server
    let handle = thread::spawn(move || {
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    // Check if the input is equal to STOP
                    if input.trim().to_uppercase() == "STOP" {
                        tx.send(1).unwrap();
                        break;
                    }
                }, 
                Err(_) => {continue;}
            }
        }
    });
    handles.push(handle);    
        
    // Listening For Connections
    for stream in listener.incoming() {
        match stream {
            Err(_) => {
                if rx.try_recv().ok() == Some(1) {
                    println!("Heard Request");
                    break;
                }
            }
            Ok(mut ok_stream) => {
                let handle = thread::spawn(move || {
                    handle_request(ok_stream);
                });
                handles.push(handle);
            }
        }
    }


    // Joining threads to the main thread
    println!("Shutting Down Server...");
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
