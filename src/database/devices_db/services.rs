use super::super::util;
use super::{DeviceData, DATABASE_NAME, TABLE_NAME};

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
    conn.execute(&command, []);
}
