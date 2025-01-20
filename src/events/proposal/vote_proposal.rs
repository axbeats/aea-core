use super::*;

// VoteProposalEvent
#[near(serializers = [json])]
#[derive(Debug)]
pub struct VoteProposalEvent {
    pub vote: ProposalVote,
}

impl VoteProposalEvent {
    pub fn emit(self) {
        let event = ProposalEvent::new(ProposalEventKind::VoteProposal(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for VoteProposalEvent {
    fn event_kind(&self) -> &str {
        "vote_proposal"
    }
}