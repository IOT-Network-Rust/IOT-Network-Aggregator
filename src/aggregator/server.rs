use super::connection::DeviceConnection;
use std::io;
use std::net::TcpListener;
use std::thread;
use crate::database::devices_db;

/// IOT server
pub struct IotServer {
    listener: TcpListener,
    handles: Vec<thread::JoinHandle<()>>,
    addr: String,
}
impl IotServer {
    /// Create a IOT tcp server
    pub fn open(addr: String) -> io::Result<Self> {
        // Binding to server can throw error
        let listener = TcpListener::bind(&addr)?;
        let handles = vec![];

        // Returning server instance
        Ok(IotServer {
            listener,
            handles,
            addr,
        })
    }

    /// Start listening on server addr
    pub fn listen(&mut self) {
        println!("Starting IOT Server On: {}", self.addr);
        
        devices_db::initialize_database();

        // Creating listener for server shutdown
        self.shutdown_signal(); // type: ignore

        // Starting to listen on stream
        println!("Server Has Started");
        for stream in self.listener.incoming() {
            // Testing if connection is ok
            if let Ok(ok_stream) = stream {
                println!("Received Connection");

                // Spawning thread to handle connection
                let handle = thread::spawn(move || {
                    let mut device = DeviceConnection::new(ok_stream).unwrap();
                    device.listen();
                });

                // Storing thread in server handles
                self.handles.push(handle);
            }
        }
    }

    /// Listener for server shutdown
    async fn shutdown_signal(&mut self) {
        // Waiting for ctrl_c shutdown
        if let Ok(_) = tokio::signal::ctrl_c().await {
            println!("Shutting Down Server...");

            // Closing threads
            for handle in self.handles.drain(..) {
                handle.join().unwrap();
            }

            println!("Server Shut Down");
        }
    }
}
