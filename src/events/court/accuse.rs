use super::*;

// Event for an accusation
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AccuseEvent {
    pub accusation: Accusation,
}

impl AccuseEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::Accuse(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for AccuseEvent {
    fn event_kind(&self) -> &str {
        "accuse"
    }
}