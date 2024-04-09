#![allow(unused)]
pub mod types;
mod tables;
use tables::Table;


pub use rusqlite::{Connection, Result};
use std::fs;
use crate::message::ConnectMSG;
use std::path;


struct DataBase {
    db_conn: Connection,
    tables: Vec<Table>
}

impl DataBase {
    pub fn open(data: &ConnectMSG) {

    }
}

const DB_FOLDER:&str = "dbs";
pub struct IotDataBase {
    tables: Vec<Table>,
    conn: Connection,
}

impl IotDataBase {
    pub fn open(data: &ConnectMSG) -> Self {
        let binding = path::Path::new(DB_FOLDER).join(format!("{}{}.db", data.name, data.id));
        let file_path = binding.to_str().unwrap();
        let conn = Connection::open(file_path).unwrap();
        
        let mut tables:Vec<Table> = vec![];
        for sensor in &data.sensors {
            tables.push(Table::new(sensor.label.clone(), sensor.data_type.as_ref().to_owned()))
        }
        
        let s = IotDataBase {
            tables,
            conn,
        };
        s.init_tables();
        s
    }

    pub fn init_tables(&self) {
        for table in &self.tables {
            table.create_table(&self.conn);
        }
    }

    pub fn insert_into_db(&self, table_name:String, data:String) {
        for table in &self.tables {
            if table.table_name == table_name {
                table.insert(&self.conn, data);
                break;
            }
        }
    }
}
