use rusqlite::ffi::Error;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use crate::database::types;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    pub label: String,
    pub data_type: types::OutputType,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub label: String,
    pub data_type: types::InputType,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectMSG {
    pub name: String,
    pub id: String,
    pub sensors: Vec<Sensor>,
    pub inputs: Vec<Input>,
}

pub fn parse_connect(s: &str) -> serde_json::Result<ConnectMSG> {
    serde_json::from_str(s)
}
