#![allow(unused)]

pub use rusqlite::{Connection, Result};
use std::fs;
use crate::message::ConnectMSG;
use std::path;


const DB_FOLDER:&str = "dbs";
pub struct IotDataBase {
    tables: Vec<String>,
    db_conn: Connection,
}

impl IotDataBase {
    pub fn open(data: &ConnectMSG) -> Self {
        let binding = path::Path::new(DB_FOLDER).join(format!("{}{}.db", data.name, data.id));
        let file_path = binding.to_str().unwrap();
        let conn = Connection::open(file_path).unwrap();
        let mut sensor_labels = vec![];
        for sensor in &data.sensors {
            sensor_labels.push(sensor.label.clone());
        }

        IotDataBase {
            tables: sensor_labels,
            db_conn: conn,
        }
    }

    pub fn open_table(&mut self, table_name:String) {
        let command = format!("CREATE TABLE {}", table_name);
        self.db_conn.execute(command.as_str(), ());
    }

    pub fn write(&self, table: String, data: String) {
        // Checks if table exists

        // Inputs data according to specified way
    }
}
