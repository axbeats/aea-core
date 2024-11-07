use super::*;
use crate::*;

pub use self::accuse::*;
pub use self::create_indictment::*;
pub use self::create_law::*;
pub use self::remove_law::*;
pub use self::retract_accusation::*;
pub use self::update_indictment_status::*;
pub use self::vote_indictment::*;

mod accuse;
mod create_indictment;
mod create_law;
mod remove_law;
mod retract_accusation;
mod update_indictment_status;
mod vote_indictment;

// Define the event variants for court events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum CourtEventKind {
    CreateLaw(CreateLawEvent),
    RemoveLaw(RemoveLawEvent),
    Accuse(AccuseEvent),
    RetractAccusation(RetractAccusationEvent),
    CreateIndictment(CreateIndictmentEvent),
    VoteIndictment(VoteIndictmentEvent),
    UpdateIndictmentStatus(UpdateIndictmentStatusEvent),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CourtEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: CourtEventKind,
}

impl CourtEvent {
    pub fn new(event: CourtEventKind) -> Self {
        CourtEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for CourtEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}