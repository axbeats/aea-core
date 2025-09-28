use crate::*;
use near_sdk::json_types::U128;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalInput {
    // Proposal fields
    pub dao_id: AccountId,
    pub proposer_id: AccountId,  // Account creating the proposal
    pub action: ProposalAction,
    pub bond: U128,
    // Video fields
    pub title: String,
    pub description: Option<String>,
    pub video_media: VideoMedia,
    pub user_location: Option<Point>,  // User's coordinates for region verification
}