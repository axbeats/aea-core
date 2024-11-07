use crate::*;
use super::*;

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

// Event for voting on an indictment
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct VoteIndictmentEvent {
    pub vote: IndictmentVote,
}

impl VoteIndictmentEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::VoteIndictment(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for VoteIndictmentEvent {
    fn event_kind(&self) -> &str {
        "vote_indictment"
    }
}

// Event for updating the status of an indictment
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateIndictmentStatusEvent {
    pub indictment_id: IndictmentId,
    pub status: IndictmentStatus,
    pub timestamp: u64,
}

impl UpdateIndictmentStatusEvent {
    pub fn emit(self) {
        let event = CourtEvent::new(CourtEventKind::UpdateIndictmentStatus(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateIndictmentStatusEvent {
    fn event_kind(&self) -> &str {
        "update_indictment_status"
    }
}
