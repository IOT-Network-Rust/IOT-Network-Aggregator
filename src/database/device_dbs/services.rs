use super::super::util;
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
pub fn get_device_sensors(device_id: &String) -> Vec<SensorTable> {
    let path = &util::get_database_path(&util::get_database_name(device_id));
    let tables = util::get_database_tables(path);
    let conn = rusqlite::Connection::open(path).unwrap();

    let mut sensors: Vec<SensorTable> = vec![];
    for table in tables {
        sensors.push(SensorTable {
            name: table,
            data_type: String::from("TEXT"),
        })
    }

    sensors
}

/// Returns data from device specific sensor
pub fn get_device_sensor_data(device_id: &String, sensor_name: &String) -> Vec<Entry> {
    vec![]
}
