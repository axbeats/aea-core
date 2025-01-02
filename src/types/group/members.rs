use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupMembers {
    pub members: Vec<AccountId>,
    pub update_method: VoteMethod,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum PotentialGroupKind {
    Followers,
    Subscribers,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupInfo {
    pub is_member: bool,
    pub group_size: u64,
}

// This struct is an evolution of GroupInfo, but GroupInfo also has lightweight purpose - Sep 6 2024
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupMembership {
    pub members: Vec<AccountId>,
    pub group_size: u64,
    pub is_member: bool,
}