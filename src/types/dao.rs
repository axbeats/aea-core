use crate::*;

pub type DaoId = AccountId;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DaoInput {
    pub profile: ProfileInput,
    pub default_policy: Policy,
    pub groups: Vec<GroupInput>,
    pub contracts: Vec<ContractInput>,
    pub rules: Vec<RuleInput>,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum DaoSetupPhase {
    Init,
    Profile,
    Groups,
    Contracts,
    Rules,
    Active,
    Failed,
}

impl Default for DaoSetupPhase {
    fn default() -> Self {
        Self::Init
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DaoConfig {
    pub dao_id: DaoId,
    pub default_policy: Policy,
    pub token_id: Option<TokenContractId>,
    pub staking_id: Option<StakingId>,
    pub metadata: ContractMetadata,
    pub setup_phase: DaoSetupPhase,
    pub setup_error: Option<String>,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DaoOutput {
    pub dao_id: DaoId,
    pub policy: Policy,
    // pub token_id: Option<TokenContractId>,
    // pub staking_id: Option<StakingId>,
}