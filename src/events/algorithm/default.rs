use super::*;

// CreateProfileEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct UpdateDefaultAlgorithmEvent {
    pub algorithm: Algorithm,
}

impl UpdateDefaultAlgorithmEvent {
    pub fn emit(self) {
        let event = AlgorithmEvent::new(AlgorithmEventKind::UpdateDefaultAlgorithm(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateDefaultAlgorithmEvent {
    fn event_kind(&self) -> &str {
        "update_default_algorithm"
    }
}