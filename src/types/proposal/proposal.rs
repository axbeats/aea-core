use crate::*;

pub type ProposalId = u64;
pub type CurrentStage = u8; // 1 based index

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub video_id: VideoId,
    pub dao_id: AccountId,
    pub proposer_id: AccountId,
    pub proposer_group_id: GroupId,
    pub action: ProposalAction,
    pub voting_sessions: Vec<ProposalGroupVotingSession>,
    pub status: ProposalStatus,
    pub submission_time: TimestampNanoSeconds,
    pub bond: u128,
}

impl Proposal {
    pub fn from_input(id: ProposalId, video_id: VideoId, input: ProposalInput) -> Self {
        Self {
            id,
            video_id,
            dao_id: input.dao_id,
            proposer_id: env::predecessor_account_id(),
            proposer_group_id: input.group_id,
            action: input.action,
            status: ProposalStatus::Initializing, // Will update later 
            voting_sessions: Vec::new(), // Will update later
            submission_time: env::block_timestamp(),
            bond: 0,
        }
    }
}

impl Proposal {
    // Get all groups that can currently vote (all groups vote simultaneously)
    pub fn get_voting_groups(&self) -> Vec<GroupId> {
        self.voting_sessions
            .iter()
            .filter(|session| matches!(session.status, GroupVoteStatus::VoteOpen))
            .map(|session| session.group_id.clone())
            .collect()
    }

    // Check if all groups have finished voting
    pub fn all_groups_voted(&self) -> bool {
        self.voting_sessions
            .iter()
            .all(|session| matches!(session.status, GroupVoteStatus::VoteClosed(_)))
    }

    // Check if all groups approved the proposal
    pub fn all_groups_approved(&self) -> bool {
        self.voting_sessions
            .iter()
            .all(|session| matches!(session.status, GroupVoteStatus::VoteClosed(GroupVoteResult::Approved)))
    }

    // Check if any group rejected the proposal
    pub fn any_group_rejected(&self) -> bool {
        self.voting_sessions
            .iter()
            .any(|session| matches!(session.status, GroupVoteStatus::VoteClosed(GroupVoteResult::Rejected)))
    }

    // Check if any group marked as spam
    pub fn any_group_spam(&self) -> bool {
        self.voting_sessions
            .iter()
            .any(|session| matches!(session.status, GroupVoteStatus::VoteClosed(GroupVoteResult::Spam)))
    }

    // Update a specific group's voting status
    pub fn update_group_status(&mut self, group_id: &GroupId, new_status: GroupVoteStatus) {
        if let Some(session) = self.voting_sessions
            .iter_mut()
            .find(|session| session.group_id == *group_id) {
            session.status = new_status;
        }
    }
}