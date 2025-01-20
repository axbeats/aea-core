use super::*;


// CreateGroupEvent
#[near(serializers = [json])]
#[derive(Debug)]
pub struct CreateProposalEvent {
    pub proposal: Proposal,
    pub timestamp: u64,
}

impl CreateProposalEvent {
    pub fn emit(self) {
        let event = ProposalEvent::new(ProposalEventKind::CreateProposal(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateProposalEvent {
    fn event_kind(&self) -> &str {
        "create_proposal"
    }
}