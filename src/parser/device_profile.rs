
use rusqlite::ffi::Error;
use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub enum OutputType {
    Integer,
    Float,
    Boolean,
    Photo,
    Audio,
    Video,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InputType {
    Integer,
    Float,
    Boolean,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    label: String,
    data_type: OutputType,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    label: String,
    data_type: InputType,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceData {
    name: String,
    id: String,
    sensors: Vec<Sensor>,
    inputs: Vec<Input>,
}


pub fn parse_device_profile(s:&str) -> serde_json::Result<DeviceData> {
    serde_json::from_str(s)
}
