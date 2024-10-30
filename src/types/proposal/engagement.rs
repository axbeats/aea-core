use crate::*;

// Import the AddAssign trait from the standard library to allow using the add_assign method.
use std::ops::AddAssign;

/// Struct representing the engagement metrics for a proposal.
///
/// This struct contains various metrics that track how a proposal is engaged with by users, including view counts,
/// comment counts, and other interactions.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct VoteEngagement {
    pub vote_count: u64,                      // Number of votes cast for this proposal
    pub view_count: u64,                      // Number of views.
    pub view_length_in_ms: u64,               // Total view length in milliseconds.
    pub comment_count: u64,                   // Number of comments.
    pub share_count: u64,                     // Number of shares.
    pub collaboration_count: u64,             // Number of collaborations.
}

// Implement the AddAssign trait for the ProposalEngagement struct to allow adding another ProposalEngagement instance to it.
impl AddAssign for VoteEngagement {
    // Define the add_assign method which takes a mutable reference to self and another ProposalEngagement instance.
    fn add_assign(&mut self, other: Self) {
        // Add the counts from the other instance to the current instance.
        self.view_count += other.view_count;
        self.view_length_in_ms += other.view_length_in_ms;
        self.comment_count += other.comment_count;
        self.share_count += other.share_count;
        self.collaboration_count += other.collaboration_count;
    }
}

impl Default for VoteEngagement {
    fn default() -> Self {
        VoteEngagement {
            // vote_tallies: Vec::new(),
            vote_count: 0,
            view_count: 0,
            view_length_in_ms: 0,
            comment_count: 0,
            share_count: 0,
            collaboration_count: 0,
        }
    }
}