use crate::*;

pub type DaoId = AccountId;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DaoInput {
    pub profile: ProfileInput,
    pub groups: Vec<GroupInput>,
    pub default_policy: Policy,
    pub token: Option<TokenInput>,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DaoConfig {
    pub dao_id: DaoId,
    pub default_policy: Policy,
    pub token_id: Option<TokenContractId>,
    pub staking_id: Option<StakingId>,
    pub metadata: ContractMetadata,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DaoOutput {
    pub dao_id: DaoId,
    pub policy: Policy,
    pub token_id: Option<TokenContractId>,
    pub staking_id: Option<StakingId>,
}