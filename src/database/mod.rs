#![allow(unused)]

pub use rusqlite::{Connection, Result};
use std::fs;
pub mod types;
use crate::message::ConnectMSG;

pub struct IotDataBase {
    tables: Vec<String>,
    db_conn: Connection,
}

impl IotDataBase {
    pub fn open(data: ConnectMSG) -> Result<Self> {
        let file_name = format!("{}{}.db", data.name, data.id);
        // Can Fail must unwrap
        // Check if database already exists for device
        
        let conn = Connection::open(file_name)?;

        // Returns data base object
        Ok(IotDataBase {
            tables: Vec::new(),
            db_conn: conn,
        })
    }

    pub fn write(&self, table: String, data: String) {
        // Checks if table exists

        // Inputs data according to specified way
    }
}
