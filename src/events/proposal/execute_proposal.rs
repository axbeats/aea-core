use super::*;

#[near(serializers = [json])]
#[derive(Debug)]
pub struct ExecuteProposalEvent {
    pub proposal_id: u64,
    pub dao_id: AccountId,
    pub action: ProposalAction,
    pub timestamp: u64,
}

impl ExecuteProposalEvent {
    pub fn emit(self) {
        let event = ProposalEvent::new(ProposalEventKind::ExecuteProposal(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for ExecuteProposalEvent {
    fn event_kind(&self) -> &str {
        "execute_proposal"
    }
}