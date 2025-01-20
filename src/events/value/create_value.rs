use super::*;

// CreateGroupEvent
#[near(serializers = [json])]
#[derive(Debug)]
pub struct CreateValueEvent {
    pub value: GovernedValue,
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