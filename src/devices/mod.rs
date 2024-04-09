#![allow(unused)]

mod inputs;
mod sensors;

use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::database;
use crate::message::ConnectMSG;

/// Represents an IoT Device.
pub struct IotDevice {
    name: String,
    id: String,
    database: database::IotDataBase,
}

impl IotDevice {
    pub fn new(connect_msg:&ConnectMSG) -> Self {
        let db = database::IotDataBase::open(connect_msg);
        IotDevice {
            name:connect_msg.name.clone(),
            id:connect_msg.id.clone(),
            database:db,
        }
    }
}
