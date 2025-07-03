use crate::*;

// pub type ProposalActionString = String;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ProposalAction {
    AddManagedContract {
        input: AddManagedContractInputVideoOption,
    },
    CreateGroup {
        input: GroupInputVideoOption,
    },
    CreateRule {
        input: RuleInputVideoOption,
    },
    CreateValue {
        input: ValueInputVideoOption,
    },
    CreateVideo {
        input: VideoInput,
    },
    DeployContract {
        input: DeployContractInputVideoOption,
    },
    FactoryInfoUpdate {
        factory_info: FactoryInfo,
    },
    FunctionCall {
        functions: Vec<FunctionCall>,
    },
    InteractProfile {
        action: InteractProfile,
    },
    InteractVideo {
        action: InteractVideo,
    },
    RemoveGroup {
        group_id: GroupId,
    },
    RemoveRule {
        rule_id: RuleId,
    },
    RemoveValue {
        value_id: ValueId,
    },
    RemoveVideo {
        video_id: VideoId,
    },
    /// Transfers given amount of `token_id` from this DAO to `receiver_id`.
    /// If `msg` is not None, calls `ft_transfer_call` with given `msg`. Fails if this base token.
    /// For `ft_transfer` and `ft_transfer_call` `memo` is the `description` of the proposal.
    Transfer {
        input: TransferInput,
    },
    UpdateContract {
        contract_id: ContractId,
        actions: Vec<UpdateContractAction>,
    },
    UpdateDefaultPolicy {
        policies: Vec<ProposalPolicyKind>,
    },
    UpdateGroup {
        group_id: GroupId,
        actions: Vec<UpdateGroupAction>,
    },
    UpdateProfile {
        actions: Vec<UpdateProfileAction>,
    },
    UpdateRule {
        rule_id: RuleId,
        actions: Vec<UpdateRuleAction>,
    },
    UpdateValue {
        value_id: ValueId,
        actions: Vec<UpdateValueAction>,
    },
    UpdateVideo {
        video_id: VideoId,
        actions: Vec<UpdateVideoAction>,
    },
    /// Upgrade another contract, by calling method with the code from given hash from blob store.
    UpgradeContract {
        input: UpgradeContractInput,
    },
    /// Upgrade this contract with given hash from blob store.
    /// This function removes the dao from the aea ecosystem
    UpgradeSelf {
        hash: Base58CryptoHash,
    },
    /// Just a signaling vote, with no execution.
    Vote,
}

impl ProposalAction {
    /// Returns label of policy for given type of proposal.
    pub fn to_policy_label(&self) -> &str {
        match self {
            ProposalAction::AddManagedContract { .. } => "add_managed_contract",
            ProposalAction::CreateGroup { .. } => "create_group",
            ProposalAction::CreateRule { .. } => "create_rule",
            ProposalAction::CreateValue { .. } => "create_value",
            ProposalAction::CreateVideo { .. } => "create_video",
            ProposalAction::DeployContract { .. } => "deploy_contract",
            ProposalAction::FactoryInfoUpdate { .. } => "factory_info_update",
            ProposalAction::FunctionCall { .. } => "function_call",
            ProposalAction::InteractProfile { .. } => "interact_profile",
            ProposalAction::InteractVideo { .. } => "interact_video",
            ProposalAction::RemoveGroup { .. } => "remove_group",
            ProposalAction::RemoveRule { .. } => "remove_rule",
            ProposalAction::RemoveValue { .. } => "remove_value",
            ProposalAction::RemoveVideo { .. } => "remove_video",
            ProposalAction::Transfer { .. } => "transfer",
            ProposalAction::UpdateContract { .. } => "update_contract",
            ProposalAction::UpdateDefaultPolicy { .. } => "update_default_policy",
            ProposalAction::UpdateGroup { .. } => "update_group",
            ProposalAction::UpdateProfile { .. } => "update_profile",
            ProposalAction::UpdateRule { .. } => "update_rule",
            ProposalAction::UpdateValue { .. } => "update_value",
            ProposalAction::UpdateVideo { .. } => "update_video",
            ProposalAction::UpgradeContract { .. } => "upgrade_contract",
            ProposalAction::UpgradeSelf { .. } => "upgrade_self",
            ProposalAction::Vote => "vote",
        }
    }

    /// Maps each ProposalAction to its corresponding ProposalKind category
    pub fn to_proposal_kind(&self) -> ProposalKind {
        match self {
            // Admin actions - manage groups, policies, and rules
            ProposalAction::CreateGroup { .. } => ProposalKind::Admin,
            ProposalAction::RemoveGroup { .. } => ProposalKind::Admin,
            ProposalAction::UpdateGroup { .. } => ProposalKind::Admin,
            ProposalAction::UpdateDefaultPolicy { .. } => ProposalKind::Admin,
            ProposalAction::CreateRule { .. } => ProposalKind::Admin,
            ProposalAction::UpdateRule { .. } => ProposalKind::Admin,
            ProposalAction::RemoveRule { .. } => ProposalKind::Admin,
            ProposalAction::FactoryInfoUpdate { .. } => ProposalKind::Admin,
            
            // Technical actions - manage contracts and values
            ProposalAction::DeployContract { .. } => ProposalKind::Technical,
            ProposalAction::UpgradeContract { .. } => ProposalKind::Technical,
            ProposalAction::UpgradeSelf { .. } => ProposalKind::Technical,
            ProposalAction::AddManagedContract { .. } => ProposalKind::Technical,
            ProposalAction::UpdateContract { .. } => ProposalKind::Technical,
            ProposalAction::CreateValue { .. } => ProposalKind::Technical,
            ProposalAction::UpdateValue { .. } => ProposalKind::Technical,
            ProposalAction::RemoveValue { .. } => ProposalKind::Technical,
            
            // Operations actions - execute functions and transfers
            ProposalAction::FunctionCall { .. } => ProposalKind::Operations,
            ProposalAction::Transfer { .. } => ProposalKind::Operations,
            
            // Social actions - manage videos and profiles
            ProposalAction::CreateVideo { .. } => ProposalKind::Social,
            ProposalAction::UpdateVideo { .. } => ProposalKind::Social,
            ProposalAction::RemoveVideo { .. } => ProposalKind::Social,
            ProposalAction::InteractVideo { .. } => ProposalKind::Social,
            ProposalAction::UpdateProfile { .. } => ProposalKind::Social,
            ProposalAction::InteractProfile { .. } => ProposalKind::Social,
            ProposalAction::Vote => ProposalKind::Social,
        }
    }
}