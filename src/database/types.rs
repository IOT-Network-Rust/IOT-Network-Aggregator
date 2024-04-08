use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub enum InputType {
    BOOLEAN,
    INTEGER,
    FLOAT,
}

#[derive(Serialize, Deserialize, Debug)]

pub enum OutputType {
    BOOLEAN,
    INTEGER,
    FLOAT,
}
