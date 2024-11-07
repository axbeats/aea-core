use super::*;
use crate::*;

pub use self::create_value::*;
pub use self::delete_value::*;
pub use self::update_value::*;
pub use self::use_value::*;

mod create_value;
mod delete_value;
mod update_value;
mod use_value;

// Define the event variants for group events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum ValueEventKind {
    CreateValue(CreateValueEvent),
    UpdateValue(UpdateValueEvent),
    DeleteValue(DeleteValueEvent),
    UseValue(UseValueEvent),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ValueEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: ValueEventKind,
}

impl ValueEvent {
    pub fn new(event: ValueEventKind) -> Self {
        ValueEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for ValueEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}