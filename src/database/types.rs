use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub enum InputType {
    TEXT,
    INTEGER,
    REAL,
}

#[derive(Serialize, Deserialize, Debug)]

pub enum OutputType {
    TEXT,
    INTEGER,
    REAL,
}
