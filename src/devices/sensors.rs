#![allow(unused)]
use rusqlite::Connection;

/// Represents a sensor.
pub struct Sensor {
    label: String, // What does the data represent
    database: Connection,
}

impl Sensor {
    pub fn new(label: String, database: Connection) -> Self {
        Sensor { label, database }
    }
}
