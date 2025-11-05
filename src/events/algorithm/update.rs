use super::*;

/// Event emitted when a user updates their algorithm
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct UpdateAlgorithmEvent {
    pub account_id: AccountId,
    pub algorithm: Algorithm,
}

impl UpdateAlgorithmEvent {
    pub fn emit(self) {
        let event = AlgorithmEvent::new(AlgorithmEventKind::UpdateAlgorithm(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateAlgorithmEvent {
    fn event_kind(&self) -> &str {
        "update_algorithm"
    }
}