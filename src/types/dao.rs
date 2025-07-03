use crate::*;
use near_sdk::serde_json::{self, Value};

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

impl DaoInput {
    /// Generate a complete example DaoInput for documentation and testing
    pub fn example() -> Self {
        Self {
            profile: ProfileInput::example(),
            default_policy: Policy::example(),
            groups: vec![
                GroupInput::example_followers(),
                GroupInput::example_token(),
                GroupInput::example_elected(),
            ],
            contracts: vec![ContractInput::example()],
            rules: vec![RuleInput::example()],
        }
    }

    /// Generate JSON representation of the example
    pub fn example_json() -> Value {
        serde_json::to_value(Self::example()).unwrap()
    }

    /// Generate a pretty-printed JSON string of the example
    pub fn example_json_string() -> String {
        serde_json::to_string_pretty(&Self::example()).unwrap()
    }
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