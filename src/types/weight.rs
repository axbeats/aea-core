use crate::*;

pub type Weight = u128;

/// Todo: Delete this, replaced with VoteWeightKind
/// Will need to refactor process_vote()
/// Aug 6 2024
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug, Hash)]
#[serde(crate = "near_sdk::serde")]
pub enum WeightKind {
    Account,        // 1 vote per account
    LinearToken,    // 1 vote per token
    QuadraticToken, // n votes per n^2 tokens
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteWeightKind {
    Single,   // 1 vote per account
    Token((StakingId, WeightFormula)),    // votes per tokens    // I also have StakingId saved in GroupKind::Token (might be a way to amalgomate) - Aug 13 2024
    Follower(WeightFormula), // votes per followers
    Distance(WeightFormula), // The distance from a point
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WeightInfo {
    pub account_weight: u128,
    pub total_weight: u128,
}