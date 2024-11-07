use super::*;

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