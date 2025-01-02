use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceVoteTally {
    pub candidate: String,
    pub count: u128,  // Adjust if needed for your use case
}