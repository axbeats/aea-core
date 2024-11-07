use super::*;

// Event for creating an indictment
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateIndictmentEvent {
    pub indictment: Indictment,
}

impl CreateIndictmentEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::CreateIndictment(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateIndictmentEvent {
    fn event_kind(&self) -> &str {
        "create_indictment"
    }
}