use rusqlite::Connection;

/// Represents a sensor.
pub struct Sensor {
    label: String, // What does the data represent
    database: Connection,
}

impl Sensor {
    
}
