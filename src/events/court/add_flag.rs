use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AddFlagEvent {
    pub flag: Flag,
}

impl AddFlagEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::AddFlag(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for AddFlagEvent {
    fn event_kind(&self) -> &str {
        "add_flag"
    }
}