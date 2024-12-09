use near_sdk::serde_json;
// use neo4rs::Node;

use crate::*;

pub type ChoiceVoteId = u64;

// There is only 1 id in this struct where it may benefit from:
// ChoiceId
// ChoiceVoteId
// Oct 2 2024
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceVote {
    pub id: ChoiceId, // Should rename this to choice_id - Nov 9 2024
    pub vote_id: ChoiceVoteId, 
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub voted_options: HashSet<CandidateId>,
    pub weight: u128,
    pub issued_at: u64,
    pub updated_at: Option<Vec<u64>>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceVoteInput {
    pub id: ChoiceId,
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub voted_options: Vec<CandidateId>,
}

impl ChoiceVote {
    pub fn from_input(input: ChoiceVoteInput, weight: u128) -> Self {
        Self {
            id: input.id,
            vote_id: 0, // Will set in contract
            account_id: input.account_id,
            dao_id: input.dao_id,
            group_id: input.group_id,
            voted_options: input.voted_options.into_iter().collect(), // Convert Vec to HashSet
            weight,
            issued_at: env::block_timestamp(), // Current block timestamp
            updated_at: None, // Initially, no updates
        }
    }
}