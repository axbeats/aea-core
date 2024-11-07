use super::*;

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
