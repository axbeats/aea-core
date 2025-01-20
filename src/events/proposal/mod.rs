use super::*;
use crate::*;

pub use self::create_proposal::*;
pub use self::delete_proposal::*;
pub use self::execute_proposal::*;
pub use self::update_proposal_status::*;
pub use self::vote_proposal::*;

mod create_proposal;
mod delete_proposal;
mod execute_proposal;
mod update_proposal_status;
mod vote_proposal;

// Define the event variants for group events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum ProposalEventKind {
    CreateProposal(CreateProposalEvent),
    VoteProposal(VoteProposalEvent),
    UpdateProposalStatus(UpdateProposalStatusEvent),
    DeleteProposal(DeleteProposalEvent),
    ExecuteProposal(ExecuteProposalEvent),  // New variant
}

#[near(serializers = [json])]
#[derive(Debug)]
pub struct ProposalEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: ProposalEventKind,
}

impl ProposalEvent {
    pub fn new(event: ProposalEventKind) -> Self {
        ProposalEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for ProposalEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}