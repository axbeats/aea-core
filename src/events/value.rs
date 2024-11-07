use super::*;
use crate::*;

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

// CreateGroupEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateValueEvent {
    pub value: Value,
    pub timestamp: u64,
}

impl CreateValueEvent {
    pub fn emit(self) {
        let event = ValueEvent::new(ValueEventKind::CreateValue(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateValueEvent {
    fn event_kind(&self) -> &str {
        "create_value"
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateValueEvent {
    pub value: Value,
    pub timestamp: u64,
}

impl UpdateValueEvent {
    pub fn emit(self) {
        let event = ValueEvent::new(ValueEventKind::UpdateValue(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateValueEvent {
    fn event_kind(&self) -> &str {
        "update_value"
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DeleteValueEvent {
    pub value_id: u64,
    pub timestamp: u64,
}

impl DeleteValueEvent {
    pub fn emit(self) {
        let event = ValueEvent::new(ValueEventKind::DeleteValue(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for DeleteValueEvent {
    fn event_kind(&self) -> &str {
        "delete_value"
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UseValueEvent {
    pub value_id: u64,
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl UseValueEvent {
    pub fn emit(self) {
        let event = ValueEvent::new(ValueEventKind::UseValue(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UseValueEvent {
    fn event_kind(&self) -> &str {
        "use_value"
    }
}
