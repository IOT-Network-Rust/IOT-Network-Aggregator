#![allow(unused)]

use rusqlite::{Connection, Result};
use std::fs;


struct IotDataBase {
    name: String,
    tables: Vec<String>,
    db_conn: Connection,
}

impl IotDataBase {
    pub fn new(device_name: String, db_path: String) -> Result<Self> { // Can Fail must unwrap
        // Check if database already exists for device
        if fs::metadata(&db_path).is_ok() {
            //Return object based off this db
            // But checks if the dbs match table wise and checks
            // for conflicts
        } else {
            // Else make db based off device sent data
        }
        let conn = Connection::open(db_path)?;
        
        // Returns data base object
        Ok(IotDataBase {
            name: device_name,
            tables: Vec::new(),
            db_conn: conn,
        })
    }

    pub fn write(&self, table: String, data: String) {
        // Checks if table exists

        // Inputs data according to specified way 

    }
}
