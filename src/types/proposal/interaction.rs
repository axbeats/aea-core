use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalInteraction {
    pub can_vote: bool,
    pub vote_count: Vec<(GroupId, [u128; 3])>,
    pub dao_id: AccountId,
}