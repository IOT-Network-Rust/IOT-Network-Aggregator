use super::super::util;
use failure::{format_err, Error};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SensorTable {
    pub name: String,
    pub data_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub value: String,
    pub time: String,
}

/// Returns a list of device sensor data
/// Helps to know what to expect from sensor
pub fn get_device_sensors(device_id: &String) -> Result<Vec<SensorTable>, Error> {
    // Getting path
    let path = &util::get_database_path(&util::get_database_name(device_id));

    // Check if path exists
    if !util::exists(path) {
        return Err(format_err!("Path '{:?}' does not exist", path));
    }

    // Opening connection
    let tables = util::get_database_tables(path);

    // Converting table data into Sensor Table
    let mut sensors: Vec<SensorTable> = vec![];
    for table in tables {
        sensors.push(SensorTable {
            name: table,
            data_type: String::from("TEXT"),
        })
    }

    Ok(sensors)
}

/// Returns data from device specific sensor
pub fn get_device_sensor_data(
    device_id: &String,
    sensor_name: &String,
) -> Result<Vec<Entry>, Error> {
    let name = util::get_database_name(device_id);
    let path = &util::get_database_path(&name);

    // Testing if path exists
    if !util::exists(path) {
        return Err(format_err!("Path '{:?}' does not exist", path));
    }

    // Testing if table exists
    let tables: Vec<String> = util::get_database_tables(path);
    if !tables.contains(sensor_name) {
        return Err(format_err!("Sensor '{}' does not exist", sensor_name));
    }

    // Opening connection
    let conn = util::open_connection(&name);
    if let Err(_) = conn {
        return Err(format_err!("Could not connect to database"));
    }

    let conn = conn.unwrap();

    // Preparing statement
    let stmt = conn.prepare(&format!("SELECT data, timestamp FROM {}", sensor_name));
    if let Err(_) = stmt {
        return Err(format_err!("Could not process command"));
    };
    let mut stmt = stmt.unwrap();

    // Creating iterator over table
    let entry_iter = stmt.query_map([], |row| {
        Ok(Entry {
            value: row.get(0)?,
            time: row.get(1)?,
        })
    })?;

    // Gathering all data
    let mut entries = vec![];
    for entry in entry_iter {
        let entry = entry.unwrap();
        entries.push(entry);
    }

    Ok(entries)
}
