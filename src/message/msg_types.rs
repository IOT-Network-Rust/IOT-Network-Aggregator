
use crate::message::parser::{
    ConnectMSG,
    UpdateMSG,
    parse_connect,
    parse_update
};



#[derive(Debug)]
#[derive(strum_macros::AsRefStr)]
pub enum Message {
    PING,
    UPDATE(UpdateMSG),
    CONNECT(ConnectMSG),
    RESPONSE(String),
}

impl Message {
    pub fn parse(string: &str) -> Option<Self> {
        let split:Vec<&str> = string.split("|").collect();
        let msg_type = *split.get(0)?;
        let data = *split.get(1)?;

        match msg_type {
            "PING" => {
                Some(Message::PING)
            },
            "UPDATE" => {
                let data = parse_update(data).unwrap();
                Some(Message::UPDATE(data))
            },
            "CONNECT" => {
                let data = parse_connect(data).unwrap();
                Some(Message::CONNECT(data))
            },
            "Response" => {
                Some(Message::RESPONSE(data.to_string()))
            },
            _ => None
        }
    }

    pub fn get_data(&self) -> String {
        match self {
            Message::PING => "None".to_string(),
            Message::CONNECT(data) => format!("{:?}", data),
            Message::RESPONSE(data) => data.clone(),
            Message::UPDATE(data) => format!("{:?}", data),
            _ => "None".to_string(),
        }
    }
}
