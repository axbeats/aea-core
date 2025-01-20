use super::*;

// VoteChoiceEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
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