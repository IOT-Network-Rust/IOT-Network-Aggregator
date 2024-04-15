use super::error;
use super::util;
use rusqlite::Connection;
use serde_derive::{Deserialize, Serialize};
pub mod services;

const DATABASE_NAME: &str = "device_catalog";
const TABLE_NAME: &str = "devices";
/// This is here to help track devices so we know what device is what
#[derive(Serialize, Deserialize)]

pub struct DeviceData {
    id: String,
    name: String,
}

impl DeviceData {
    pub fn new(name: &String, id: &String) -> Self {
        DeviceData {
            name: name.clone(),
            id: id.clone(),
        }
    }
}

pub fn initialize_database() {
    let path = util::get_database_path(&util::get_database_name(&DATABASE_NAME.to_string()));
    let tables = util::get_database_tables(&path);
    match tables.get(0) {
        Some(_) => (),
        None => {
            let conn = Connection::open(path).expect(error::FAILURE_TO_OPEN);

            let command = format!(
                "CREATE TABLE {} (
                        id    INTEGER PRIMARY KEY,
                        name  TEXT NOT NULL,
                        device_id TEXT NOT NULL,
                        UNIQUE(device_id)
                    )",
                TABLE_NAME
            );

            conn.execute(&command, []).expect(error::FAILURE_TO_OPEN);

            conn.close().expect(error::FAILURE_TO_CLOSE);
        }
    }
}

pub fn add_device(device: DeviceData) {
    let path = util::get_database_path(&util::get_database_name(&DATABASE_NAME.to_string()));
    let conn = Connection::open(path).expect(error::FAILURE_TO_OPEN);

    let command = format!(
        "INSERT OR REPLACE INTO {} (name, device_id) VALUES (?1, ?2)",
        TABLE_NAME
    );

    println!("{}", command);

    println!("{}, {}", device.name, device.id);
    conn.execute(&command, (device.name, device.id))
        .expect(error::FAILURE_TO_INSERT);

    conn.close().expect(error::FAILURE_TO_CLOSE);
}
