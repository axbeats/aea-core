use crate::*;
use near_sdk::json_types::U128;

pub type ProposalId = u64;
pub type CurrentStage = u8; // 1 based index

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub video_id: VideoId,
    pub dao_id: AccountId,
    pub proposer_id: AccountId,
    pub proposer_role_id: RoleId,
    pub action: ProposalAction,
    pub voting_sessions: Vec<ProposalRoleVotingSession>,
    pub status: ProposalStatus,
    pub submission_time: TimestampNanoSeconds,
    pub bond: U128,
}

impl Proposal {
    pub fn from_input(id: ProposalId, video_id: VideoId, input: ProposalInput) -> Self {
        Self {
            id,
            video_id,
            dao_id: input.dao_id,
            proposer_id: env::predecessor_account_id(),
            proposer_role_id: input.role_id,
            action: input.action,
            status: ProposalStatus::Initializing, // Will update later 
            voting_sessions: Vec::new(), // Will update later
            submission_time: env::block_timestamp(),
            bond: U128(0),
        }
    }
}

impl Proposal {
    // Get all roles that can currently vote (all roles vote simultaneously)
    pub fn get_voting_roles(&self) -> Vec<RoleId> {
        self.voting_sessions
            .iter()
            .filter(|session| matches!(session.status, RoleVoteStatus::VoteOpen))
            .map(|session| session.role_id.clone())
            .collect()
    }

    // Check if all roles have finished voting
    pub fn all_roles_voted(&self) -> bool {
        self.voting_sessions
            .iter()
            .all(|session| matches!(session.status, RoleVoteStatus::VoteClosed(_)))
    }

    // Check if all roles approved the proposal
    pub fn all_roles_approved(&self) -> bool {
        self.voting_sessions
            .iter()
            .all(|session| matches!(session.status, RoleVoteStatus::VoteClosed(RoleVoteResult::Approved)))
    }

    // Check if any role rejected the proposal
    pub fn any_role_rejected(&self) -> bool {
        self.voting_sessions
            .iter()
            .any(|session| matches!(session.status, RoleVoteStatus::VoteClosed(RoleVoteResult::Rejected)))
    }

    // Check if any role marked as spam
    pub fn any_role_spam(&self) -> bool {
        self.voting_sessions
            .iter()
            .any(|session| matches!(session.status, RoleVoteStatus::VoteClosed(RoleVoteResult::Spam)))
    }

    // Update a specific role's voting status
    pub fn update_role_status(&mut self, role_id: &RoleId, new_status: RoleVoteStatus) {
        if let Some(session) = self.voting_sessions
            .iter_mut()
            .find(|session| session.role_id == *role_id) {
            session.status = new_status;
        }
    }
}