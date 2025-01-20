use super::*;

#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct RemoveFlagEvent {
    pub flag_id: FlagId,
    pub timestamp: TimestampNanoSeconds,
}

impl RemoveFlagEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::RemoveFlag(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveFlagEvent {
    fn event_kind(&self) -> &str {
        "remove_flag"
    }
}