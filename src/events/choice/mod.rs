use super::*;
use crate::*;

pub use self::create_choice::*;
pub use self::delete_choice::*;
pub use self::update_choice::*;
pub use self::update_elected_options::*;
pub use self::vote_choice::*;

mod create_choice;
mod delete_choice;
mod update_choice;
mod update_elected_options;
mod vote_choice;

// Define the event variants for choice events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum ChoiceEventKind {
    CreateChoice(CreateChoiceEvent),
    DeleteChoice(DeleteChoiceEvent),
    UpdateChoice(UpdateChoiceEvent),
    UpdateElectedOptions(UpdateElectedOptionsEvent),
    VoteChoice(VoteChoiceEvent),
}

#[near(serializers = [json])]
#[derive(Debug)]
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