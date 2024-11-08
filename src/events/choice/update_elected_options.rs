use super::*;

// NOTE: Might want to handle this event in the values contract - Nov 7 2024

// UpdateElectedOptionsEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateElectedOptionsEvent {
    pub choice_id: ChoiceId,
    pub timestamp: u64,
}

impl UpdateElectedOptionsEvent {
    pub fn emit(self) {
        let event = ChoiceEvent::new(ChoiceEventKind::UpdateElectedOptions(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateElectedOptionsEvent {
    fn event_kind(&self) -> &str {
        "update_elected_options"
    }
}