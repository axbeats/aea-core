use super::*;

// CreateChoiceEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateChoiceEvent {
    pub choice: Choice,
    // pub options: Vec<(OptionId, ValueType)>,  // Include option IDs and their values
}


impl CreateChoiceEvent {
    pub fn emit(self) {
        let event = ChoiceEvent::new(ChoiceEventKind::CreateChoice(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateChoiceEvent {
    fn event_kind(&self) -> &str {
        "create_choice"
    }
}