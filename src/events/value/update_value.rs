use super::*;

#[near(serializers = [json])]
#[derive(Debug)]
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