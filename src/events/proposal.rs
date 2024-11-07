use crate::*;
use super::*;

// Define the event variants for group events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalEventKind {
    CreateProposal(CreateProposalEvent),
    VoteProposal(VoteProposalEvent),
    UpdateProposalStatus(UpdateProposalStatusEvent),
    DeleteProposal(DeleteProposalEvent),
    ExecuteProposal(ExecuteProposalEvent),  // New variant
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// CreateGroupEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// VoteProposalEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// UpdateProposalStatusEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExecuteProposalEvent {
    pub proposal_id: u64,
    pub dao_id: AccountId,
    pub kind: ProposalKind,
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
