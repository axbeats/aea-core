use super::*;

// Event for retracting an accusation
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RetractAccusationEvent {
    pub accusation_id: AccusationId,
    pub retractor_id: AccountId,
    pub timestamp: u64,
}

impl RetractAccusationEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::RetractAccusation(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RetractAccusationEvent {
    fn event_kind(&self) -> &str {
        "retract_accusation"
    }
}