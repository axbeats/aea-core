use super::*;

// Event for creating a law
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateLawEvent {
    pub law: Law,
}

impl CreateLawEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::CreateLaw(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateLawEvent {
    fn event_kind(&self) -> &str {
        "create_law"
    }
}