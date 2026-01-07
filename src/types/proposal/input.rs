use crate::*;
use aea_macros::Generable;
use near_sdk::json_types::U128;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Generable)]
pub struct ProposalInput {
    // Proposal fields
    pub dao_id: AccountId,
    pub proposer_id: AccountId,  // Account creating the proposal
    pub action: ProposalAction,
    pub bond: U128,
    // Video fields
    pub caption: Option<String>,
    pub video_media: VideoMedia,
    pub user_location: Option<Point>,  // User's coordinates for region verification
}

impl Default for ProposalInput {
    fn default() -> Self {
        Self {
            dao_id: "example.near".parse().unwrap(),
            proposer_id: "proposer.near".parse().unwrap(),
            action: ProposalAction::default(),
            bond: U128(0),
            caption: None,
            video_media: VideoMedia::default(),
            user_location: None,
        }
    }
}