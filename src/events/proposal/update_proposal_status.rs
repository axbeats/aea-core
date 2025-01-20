use super::*;

// UpdateProposalStatusEvent
#[near(serializers = [json])]
#[derive(Debug)]
pub struct UpdateProposalStatusEvent {
    pub proposal_id: u64,
    pub status: ProposalStatus,
    pub timestamp: u64,
}

impl UpdateProposalStatusEvent {
    pub fn emit(self) {
        let event = ProposalEvent::new(ProposalEventKind::UpdateProposalStatus(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateProposalStatusEvent {
    fn event_kind(&self) -> &str {
        "update_proposal_status"
    }
}