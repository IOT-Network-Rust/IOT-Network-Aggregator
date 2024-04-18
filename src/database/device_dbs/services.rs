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

    if util::exists(path) {
        let tables = util::get_database_tables(path);
        let conn = rusqlite::Connection::open(path).unwrap();

        let mut sensors: Vec<SensorTable> = vec![];
        for table in tables {
            sensors.push(SensorTable {
                name: table,
                data_type: String::from("TEXT"),
            })
        }
        return sensors;
    }
    vec![]
}

/// Returns data from device specific sensor
pub fn get_device_sensor_data(
    device_id: &String,
    sensor_name: &String,
) -> Result<Vec<Entry>, rusqlite::Error> {
    let name = util::get_database_name(device_id);
    let path = &util::get_database_path(&name);

    // Testing if path exists
    if !util::exists(path) {
        println!("Could not find device: {}", device_id);
        return Err(rusqlite::Error::ExecuteReturnedResults);
    }

    // Testing if table exists
    let tables: Vec<String> = util::get_database_tables(path);
    if !tables.contains(sensor_name) {
        println!("Could not find table: {}", sensor_name);
        return Err(rusqlite::Error::ExecuteReturnedResults);
    }

    // Opening connection
    let conn = util::open_connection(&name);
    if let Err(e) = conn {
        println!("Connection could not be made:");
        return Err(rusqlite::Error::ExecuteReturnedResults)
    }

    let conn = conn.unwrap();

    // Preparing statement
    println!("{}", sensor_name);
    let mut stmt = conn.prepare(&format!("SELECT data, timestamp FROM {}", sensor_name));
    if let Err(e) = stmt {
        println!("There was an issue processing command to db");
        return Err(e)
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
        println!("{}", &entry.time);

        entries.push(entry);
    }

    Ok(entries)
}
