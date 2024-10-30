use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MutualConnections {
    pub followers: Vec<AccountId>,
    pub following: Vec<AccountId>,
}