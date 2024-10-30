use crate::*;

pub type DaoId = AccountId;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DaoInput {
    pub profile: ProfileInput,
    pub groups: Vec<GroupInput>,
    pub default_policy: Policy,
    pub token: Option<TokenInput>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DaoConfig {
    pub dao_id: DaoId,
    pub default_policy: Policy,
    pub token_id: Option<TokenId>,
    pub staking_id: Option<StakingId>,
    pub metadata: ContractMetadata,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DaoOutput {
    pub dao_id: DaoId,
    pub policy: Policy,
    pub token_id: Option<TokenId>,
    pub staking_id: Option<StakingId>,
}