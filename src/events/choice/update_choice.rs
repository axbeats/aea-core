use super::*;

// UpdateChoiceEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct UpdateChoiceEvent {
    pub choice: Choice,
    pub timestamp: u64,
}

impl UpdateChoiceEvent {
    pub fn emit(self) {
        let event = ChoiceEvent::new(ChoiceEventKind::UpdateChoice(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateChoiceEvent {
    fn event_kind(&self) -> &str {
        "update_choice"
    }
}