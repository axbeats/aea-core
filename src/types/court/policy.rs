use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RulePolicy {
    pub flag_threshold: PercentageU32,   // Flags to views
    pub flag_quorum: u64,                // Min number of flags
    pub review_threshold: PercentageU32, // Guilty to innocent
    pub review_quorum: Quorum,           // Min number of votes
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Quorum {
    Fixed(u64),
    Percentage(Percentage),
}