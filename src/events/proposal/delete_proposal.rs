use super::*;

#[near(serializers = [json])]
#[derive(Debug)]
pub struct DeleteProposalEvent {
    pub proposal_id: u64,
    pub timestamp: u64,
}

impl DeleteProposalEvent {
    pub fn emit(self) {
        let event = ProposalEvent::new(ProposalEventKind::DeleteProposal(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for DeleteProposalEvent {
    fn event_kind(&self) -> &str {
        "delete_proposal"
    }
}