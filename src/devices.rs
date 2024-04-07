#![allow(unused)]

mod inputs;
mod sensors;

use std::io::{Read, Write};
use std::io;
use std::net::{TcpListener, TcpStream};

use crate::database;

/// Represents an IoT Device.
pub struct IotDevice {
    name: String,
    id: u32,
    database:database::IotDataBase,
}

impl IotDevice {
    pub fn new(name:String, id:u32) -> database::Result<Self> {
        let db_path = format!("{name}:{id}.db");
        let database = database::IotDataBase::open(db_path)?;
        Ok(IotDevice {
            name, 
            id,
            database,
        })
    }
}
