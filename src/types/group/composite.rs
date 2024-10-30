use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupComposite {
    pub group: Group,
    pub size: u64,
    pub members: Vec<AccountId>,
    pub is_member: bool
}