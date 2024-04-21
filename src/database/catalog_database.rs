//! Holds data about all devices
//!
//!

use super::error;
use super::util;
use rusqlite::Connection;
use serde_derive::{Deserialize, Serialize};

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

    conn.execute(&command, (device.name, device.id))
        .expect(error::FAILURE_TO_INSERT);

    conn.close().expect(error::FAILURE_TO_CLOSE);
}


/// Returns a list of devices
pub fn get_all_devices() -> Result<Vec<DeviceData>, rusqlite::Error> {
    let conn = util::open_connection(&util::get_database_name(&DATABASE_NAME.to_string())).unwrap();

    let mut stmt = conn.prepare(&format!("SELECT id, name, device_id FROM {}", TABLE_NAME))?;
    let device_iter = stmt.query_map([], |row| {
        Ok(DeviceData {
            name: row.get(1)?,
            id: row.get(2)?,
        })
    })?;

    let mut devices = Vec::new();
    for device_result in device_iter {
        devices.push(device_result?);
    }

    Ok(devices)
}

/// Returns information about device given its id
pub fn get_device(id: String) -> Result<DeviceData, rusqlite::Error> {
    let devices = get_all_devices()?;

    for device in devices {
        if device.id == id {
            return Ok(device);
        }
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

/// Remove Device
pub fn remove_device(device_id: u32) {
    let conn = util::open_connection(&util::get_database_name(&DATABASE_NAME.to_string())).unwrap();
    let command = format!("DELETE FROM {} WHERE device_id={};", TABLE_NAME, device_id);
    conn.execute(&command, []).unwrap();
}
