use super::*;

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