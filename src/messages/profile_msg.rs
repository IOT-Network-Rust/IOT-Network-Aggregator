use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    pub label: String,
    pub data_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub label: String,
    pub data_type: String,
}
/// Represents what server expects when it is sent a connect message
/// from a device.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileMSG {
    pub name: String,
    pub id: String,
    pub sensors: Vec<Sensor>,
    pub inputs: Vec<Input>,
}

/// Parser that converts str of data into a struct that represents that data.
pub fn parse_profile(s: &str) -> serde_json::Result<ProfileMSG> {
    serde_json::from_str(s)
}
