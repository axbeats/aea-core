use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(crate = "near_sdk::serde")]
pub enum CandidateId {
    U64(u64),
    String(String),
    AccountId(AccountId),
    // Add other variants as needed
}