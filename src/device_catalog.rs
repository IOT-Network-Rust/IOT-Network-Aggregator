use crate::database_handler;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

const DATABASE_NAME: &str = "device_catalog";
const TABLE_NAME: &str = "devices";
/// This is here to help track devices so we know what device is what
pub struct DeviceData {
    name:String,
    id:String,
}

impl DeviceData {
    pub fn new(name:&String, id:&String) -> Self {
        DeviceData {
            name:name.clone(),
            id:id.clone(),
        }
    }
}

pub fn initialize_database() {
    let tables = database_handler::get_tables(&DATABASE_NAME.to_string());
    match tables.get(0) {
        Some(_) => (),
        None => {
            let path = Path::new(database_handler::DATA_FOLDER).join(format!("{}.db", DATABASE_NAME));
            let conn = Connection::open(path).expect(database_handler::FAILURE_TO_OPEN);

            let command = format!(
                "CREATE TABLE {} (
                        id    INTEGER PRIMARY KEY,
                        name  TEXT NOT NULL,
                        device_id TEXT NOT NULL,
                        UNIQUE(device_id)
                    )",
                TABLE_NAME
            );

            conn.execute(&command, [])
                .expect(database_handler::FAILURE_TO_OPEN);

            conn.close()
                .expect(database_handler::FAILURE_TO_CLOSE);
        }
    }
}

pub fn add_device(device:DeviceData) {
    let path = Path::new(database_handler::DATA_FOLDER).join(format!("{}.db", DATABASE_NAME));
    let conn = Connection::open(path).expect(database_handler::FAILURE_TO_OPEN);

    let command = format!("INSERT OR REPLACE INTO {} (name, device_id) VALUES (?1, ?2)", TABLE_NAME);

    println!("{}",command);

    println!("{}, {}", device.name, device.id);
    conn.execute(&command, (device.name, device.id))
        .expect(database_handler::FAILURE_TO_INSERT);

    conn.close()
        .expect(database_handler::FAILURE_TO_CLOSE);
}

pub fn remove_device(device_id:u32) {
    let path = Path::new(database_handler::DATA_FOLDER).join(format!("{}.db", DATABASE_NAME));
    let conn = Connection::open(path).expect(database_handler::FAILURE_TO_OPEN);

    let command = format!("DELETE FROM {} WHERE device_id={};", TABLE_NAME, device_id);
    conn.execute(&command, []);
}
