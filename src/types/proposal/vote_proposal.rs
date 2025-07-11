// use neo4rs::Node;

use crate::*;

pub type ProposalVoteId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalVote {
    pub id: ProposalVoteId,
    pub proposal_id: ProposalId,
    pub ability: ProposalAbility,
    pub voter_id: AccountId,
    pub dao_id: AccountId,
    pub role_id: RoleId,
    pub vote: Vote,
    pub weight: u128,
    pub issued_at: u64,
    pub updated_at: Option<Vec<u64>>,
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
    pub ability: ProposalAbility,
    pub voter_id: AccountId,
    pub dao_id: AccountId,
    pub role_id: RoleId,
    pub vote: Vote,
    pub user_location: Option<Point>,  // User's coordinates for region verification
}

impl ProposalVoteInput {
    pub fn to_proposal_vote(
        &self,
        weight: u128,
        update: bool,
    ) -> ProposalVote {
        // Get the current timestamp
        let current_time = env::block_timestamp();

        // Create the `updated_at` field based on the `update` flag
        let updated_at = if update {
            Some(vec![current_time])
        } else {
            None
        };

        // Build and return the `ProposalVote`
        ProposalVote {
            id: 0, // Will set later
            proposal_id: self.proposal_id.clone(),
            ability: self.ability.clone(),
            voter_id: self.voter_id.clone(),
            dao_id: self.dao_id.clone(),
            role_id: self.role_id.clone(),
            vote: self.vote.clone(),
            weight,
            issued_at: current_time,
            updated_at,
        }
    }
}