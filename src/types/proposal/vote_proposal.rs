use crate::*;
use std::collections::HashMap;

pub type ProposalVoteId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalVote {
    pub id: ProposalVoteId,
    pub proposal_id: ProposalId,
    pub voter_id: AccountId,
    pub vote: Vote,
    pub roles_weights: HashMap<RoleId, u128>,  // Which roles voted and their weights
    pub total_weight: u128,  // Sum of all weights for quick access
    pub issued_at: u64,
    pub updated_at: Option<u64>,
}

/// Votes recorded in the proposal.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum Vote {
    // Approve = 0x0,
    // Reject = 0x1,
    // Spam = 0x2,
    Approve,
    Reject,
    Spam,
}


#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalVoteInput {
    pub proposal_id: ProposalId,
    pub voter_id: AccountId,
    pub vote: Vote,
    pub user_location: Option<Point>,  // User's coordinates for region verification
}

