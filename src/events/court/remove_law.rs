use super::*;

// Event for removing a law
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RemoveLawEvent {
    pub law_id: LawId,
    pub timestamp: u64,
}

impl RemoveLawEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::RemoveLaw(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveLawEvent {
    fn event_kind(&self) -> &str {
        "remove_law"
    }
}