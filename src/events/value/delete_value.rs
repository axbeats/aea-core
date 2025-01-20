use super::*;

#[near(serializers = [json])]
#[derive(Debug)]
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