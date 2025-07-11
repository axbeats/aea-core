use crate::*;

// pub type ProposalActionString = String;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ProposalAction {
    AddManagedContract {
        input: AddManagedContractInputVideoOption,
    },
    CreateRole {
        input: RoleInputVideoOption,
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
    FunctionCall {
        functions: Vec<FunctionCall>,
    },
    InteractProfile {
        action: InteractProfile,
    },
    InteractVideo {
        action: InteractVideo,
    },
    RemoveRole {
        role_id: RoleId,
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
    UpdateRole {
        role_id: RoleId,
        actions: Vec<UpdateRoleAction>,
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
            ProposalAction::CreateRole { .. } => "create_role",
            ProposalAction::CreateRule { .. } => "create_rule",
            ProposalAction::CreateValue { .. } => "create_value",
            ProposalAction::CreateVideo { .. } => "create_video",
            ProposalAction::DeployContract { .. } => "deploy_contract",
            ProposalAction::FunctionCall { .. } => "function_call",
            ProposalAction::InteractProfile { .. } => "interact_profile",
            ProposalAction::InteractVideo { .. } => "interact_video",
            ProposalAction::RemoveRole { .. } => "remove_role",
            ProposalAction::RemoveRule { .. } => "remove_rule",
            ProposalAction::RemoveValue { .. } => "remove_value",
            ProposalAction::RemoveVideo { .. } => "remove_video",
            ProposalAction::Transfer { .. } => "transfer",
            ProposalAction::UpdateContract { .. } => "update_contract",
            ProposalAction::UpdateDefaultPolicy { .. } => "update_default_policy",
            ProposalAction::UpdateRole { .. } => "update_role",
            ProposalAction::UpdateProfile { .. } => "update_profile",
            ProposalAction::UpdateRule { .. } => "update_rule",
            ProposalAction::UpdateValue { .. } => "update_value",
            ProposalAction::UpdateVideo { .. } => "update_video",
            ProposalAction::UpgradeContract { .. } => "upgrade_contract",
            ProposalAction::UpgradeSelf { .. } => "upgrade_self",
            ProposalAction::Vote => "vote",
        }
    }

    /// Maps each ProposalAction to its corresponding ProposalAbility category
    pub fn to_ability(&self) -> ProposalAbility {
        match self {
            // Admin actions - manage roles, policies, and rules
            ProposalAction::CreateRole { .. } => ProposalAbility::Role,
            ProposalAction::RemoveRole { .. } => ProposalAbility::Role,
            ProposalAction::UpdateRole { .. } => ProposalAbility::Role,

            ProposalAction::UpdateDefaultPolicy { .. } => ProposalAbility::Policy,

            ProposalAction::CreateRule { .. } => ProposalAbility::Court,
            ProposalAction::UpdateRule { .. } => ProposalAbility::Court,
            ProposalAction::RemoveRule { .. } => ProposalAbility::Court,
            
            // Technical actions - manage contracts and values
            ProposalAction::DeployContract { .. } => ProposalAbility::Code,
            ProposalAction::UpgradeContract { .. } => ProposalAbility::Code,
            ProposalAction::UpgradeSelf { .. } => ProposalAbility::Code,
            ProposalAction::AddManagedContract { .. } => ProposalAbility::Code,
            ProposalAction::UpdateContract { .. } => ProposalAbility::Code,

            ProposalAction::CreateValue { .. } => ProposalAbility::Value,
            ProposalAction::UpdateValue { .. } => ProposalAbility::Value,
            ProposalAction::RemoveValue { .. } => ProposalAbility::Value,
            
            // Operations actions - execute functions and transfers
            ProposalAction::FunctionCall { .. } => ProposalAbility::Task,
            ProposalAction::Transfer { .. } => ProposalAbility::Task,
            
            // Social actions - manage videos and profiles
            ProposalAction::CreateVideo { .. } => ProposalAbility::Video,
            ProposalAction::UpdateVideo { .. } => ProposalAbility::Video,
            ProposalAction::RemoveVideo { .. } => ProposalAbility::Video,
            ProposalAction::InteractVideo { .. } => ProposalAbility::Video,

            ProposalAction::UpdateProfile { .. } => ProposalAbility::Profile,
            ProposalAction::InteractProfile { .. } => ProposalAbility::Profile,
            ProposalAction::Vote => ProposalAbility::Profile,
        }
    }
}