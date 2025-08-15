use crate::*;
use near_sdk::json_types::U128;
use std::collections::HashMap;

pub type ProposalId = u64;
pub type CurrentStage = u8; // 1 based index

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub video_id: VideoId,
    pub dao_id: AccountId,
    pub proposer_id: AccountId,
    pub action: ProposalAction,
    pub voting_roles: HashMap<RoleId, Policy>,  // Roles that can vote and their policies
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
            proposer_id: input.proposer_id,
            action: input.action,
            status: ProposalStatus::Initializing, // Will update later 
            voting_roles: HashMap::new(), // Will be populated from policy contract
            submission_time: env::block_timestamp(),
            bond: input.bond,
        }
    }
}

impl Proposal {
    // Get all roles that can vote
    pub fn get_voting_roles(&self) -> Vec<RoleId> {
        self.voting_roles.keys().cloned().collect()
    }

    // Check if a specific role can vote
    pub fn can_role_vote(&self, role_id: &RoleId) -> bool {
        self.voting_roles.contains_key(role_id)
    }

    // Get policy for a specific role
    pub fn get_role_policy(&self, role_id: &RoleId) -> Option<&Policy> {
        self.voting_roles.get(role_id)
    }

    // Check if voting period has ended for a role
    pub fn is_voting_expired(&self, role_id: &RoleId) -> bool {
        if let Some(policy) = self.voting_roles.get(role_id) {
            let current_time = env::block_timestamp();
            let voting_end = self.submission_time + policy.voting_period;
            current_time > voting_end
        } else {
            true // If role not found, consider voting expired
        }
    }
}