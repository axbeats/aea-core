use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LawPolicy {
    pub threshold: Percentage,
    pub quorum: Quorum,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Quorum {
    Fixed(u64),
    Percentage(Percentage),
}