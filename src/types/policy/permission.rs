use crate::*;

pub type PermissionString = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalPermission {
    pub create: bool,
    pub vote: bool,
    pub order: Option<VoteOrder>,
    pub custom_policy: Option<CustomPolicy>,
}