use crate::*;

pub type ProposalKindString = String;

/// Kinds of proposals, doing different action.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ProposalKind {
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
    ElectAccount {
        group_id: GroupId,
        account_id: AccountId,
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
    UnelectAccount {
        group_id: GroupId,
        account_id: AccountId,
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

impl ProposalKind {
    /// Returns label of policy for given type of proposal.
    pub fn to_policy_label(&self) -> &str {
        match self {
            ProposalKind::AddManagedContract { .. } => "add_managed_contract",
            ProposalKind::CreateGroup { .. } => "create_group",
            ProposalKind::CreateRule { .. } => "create_rule",
            ProposalKind::CreateValue { .. } => "create_value",
            ProposalKind::CreateVideo { .. } => "create_video",
            ProposalKind::DeployContract { .. } => "deploy_contract",
            ProposalKind::ElectAccount { .. } => "elect_account",
            ProposalKind::FactoryInfoUpdate { .. } => "factory_info_update",
            ProposalKind::FunctionCall { .. } => "function_call",
            ProposalKind::InteractProfile { .. } => "interact_profile",
            ProposalKind::InteractVideo { .. } => "interact_video",
            ProposalKind::RemoveGroup { .. } => "remove_group",
            ProposalKind::RemoveRule { .. } => "remove_rule",
            ProposalKind::RemoveValue { .. } => "remove_value",
            ProposalKind::RemoveVideo { .. } => "remove_video",
            ProposalKind::Transfer { .. } => "transfer",
            ProposalKind::UnelectAccount { .. } => "unelect_account",
            ProposalKind::UpdateContract { .. } => "update_contract",
            ProposalKind::UpdateDefaultPolicy { .. } => "update_default_policy",
            ProposalKind::UpdateGroup { .. } => "update_group",
            ProposalKind::UpdateProfile { .. } => "update_profile",
            ProposalKind::UpdateRule { .. } => "update_rule",
            ProposalKind::UpdateValue { .. } => "update_value",
            ProposalKind::UpdateVideo { .. } => "update_video",
            ProposalKind::UpgradeContract { .. } => "upgrade_contract",
            ProposalKind::UpgradeSelf { .. } => "upgrade_self",
            ProposalKind::Vote => "vote",
        }
    }
}