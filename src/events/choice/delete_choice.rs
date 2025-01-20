use super::*;

// DeleteChoiceEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct DeleteChoiceEvent {
    pub choice_id: u64,
    pub timestamp: u64,
}

impl DeleteChoiceEvent {
    pub fn emit(self) {
        let event = ChoiceEvent::new(ChoiceEventKind::DeleteChoice(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for DeleteChoiceEvent {
    fn event_kind(&self) -> &str {
        "delete_choice"
    }
}