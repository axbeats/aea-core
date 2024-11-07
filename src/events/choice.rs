use crate::*;
use super::*;

// Define the event variants for choice events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum ChoiceEventKind {
    CreateChoice(CreateChoiceEvent),
    DeleteChoice(DeleteChoiceEvent),
    UpdateChoice(UpdateChoiceEvent),
    UpdateElectedOptions(UpdateElectedOptionsEvent),
    VoteChoice(VoteChoiceEvent),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: ChoiceEventKind,
}

impl ChoiceEvent {
    pub fn new(event: ChoiceEventKind) -> Self {
        ChoiceEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for ChoiceEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

// CreateChoiceEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateChoiceEvent {
    pub choice: Choice,
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

// DeleteChoiceEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DeleteChoiceEvent {
    pub choice_id: u64,
    pub timestamp: u64,
}

impl DeleteChoiceEvent {
    pub fn emit(self) {
        let event = ChoiceEvent::new(ChoiceEventKind::DeleteChoice(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for DeleteChoiceEvent {
    fn event_kind(&self) -> &str {
        "delete_choice"
    }
}

// UpdateChoiceEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateChoiceEvent {
    pub choice: Choice,
    pub timestamp: u64,
}

impl UpdateChoiceEvent {
    pub fn emit(self) {
        let event = ChoiceEvent::new(ChoiceEventKind::UpdateChoice(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateChoiceEvent {
    fn event_kind(&self) -> &str {
        "update_choice"
    }
}

// VoteChoiceEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct VoteChoiceEvent {
    pub vote: ChoiceVote,
}

impl VoteChoiceEvent {
    pub fn emit(self) {
        let event = ChoiceEvent::new(ChoiceEventKind::VoteChoice(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for VoteChoiceEvent {
    fn event_kind(&self) -> &str {
        "vote_choice"
    }
}

// UpdateElectedOptionsEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateElectedOptionsEvent {
    pub choice_id: ChoiceId,
    pub timestamp: u64,
}

impl UpdateElectedOptionsEvent {
    pub fn emit(self) {
        let event = ChoiceEvent::new(ChoiceEventKind::UpdateElectedOptions(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateElectedOptionsEvent {
    fn event_kind(&self) -> &str {
        "update_elected_options"
    }
}
