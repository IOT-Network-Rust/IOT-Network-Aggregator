use crate::database::types::{InputType, OutputType};
use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct DbEntry {
    pub table: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMSG {
    pub entries: Vec<DbEntry>,
}

pub fn parse_update(string: &str) -> serde_json::Result<UpdateMSG> {
    serde_json::from_str(string)
}
