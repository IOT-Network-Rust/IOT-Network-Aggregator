mod profile_msg;
mod update_msg;

pub use profile_msg::{parse_profile, ProfileMSG};
pub use update_msg::{parse_update, UpdateMSG};

#[derive(Debug, strum_macros::AsRefStr)]
pub enum Message {
    PING,
    UPDATE(UpdateMSG),
    PROFILE(ProfileMSG),
    RESPONSE(String),
}

impl Message {
    pub fn parse(string: &str) -> Option<Self> {
        let split: Vec<&str> = string.split("|").collect();
        let msg_type = *split.get(0)?;
        let data = *split.get(1)?;

        match msg_type {
            "PING" => Some(Message::PING),
            "UPDATE" => {
                let data = parse_update(data).unwrap();
                Some(Message::UPDATE(data))
            }
            "PROFILE" => {
                let data = parse_profile(data).unwrap();
                Some(Message::PROFILE(data))
            }
            "RESPONSE" => Some(Message::RESPONSE(data.to_string())),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Message::PING => {"PING".to_string()},
            Message::PROFILE(profile) => {format!("{:?}", profile)},
            Message::UPDATE(update) => {format!("{:?}", update)},
            Message::RESPONSE(string) => {string.clone()}

        }
    }
}
