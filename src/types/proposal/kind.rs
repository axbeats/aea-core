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
    RemoveGroup {
        group_id: GroupId,
    },
    RemoveRule {
        rule_id: RuleId,
    },
    RemoveValue {
        value_id: ValueId,
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
    UpdateChoiceMetadata {
        choice_id: ChoiceId,
        description: Option<String>,
        video: Option<VideoHash>,
        image: Option<ImageHash>,
    },
    UpdateChoiceSize {
        choice_id: ChoiceId,
        size: u8,
    },
    UpdateGroupName {
        group_id: GroupId,
        name: String,
    },
    UpdateGroupPermissions {
        group_id: GroupId,
        permissions: HashMap<ProposalKindString, ProposalPermission>,
    },
    UpdateGroupVoteMethod {
        group_id: GroupId,
        vote_method: VoteMethod,
    },
    UpdateDefaultPolicy {
        kind: ProposalPolicyKind,
    },
    UpdateRule {
        rule_id: RuleId, // Need more - Feb 26 2025
    },
    UpdateValueName {
        value_id: ValueId,
        name: String,
    },
    UpdateValue {
        value: Value,
    },
    UpdateValueVoteMethod {
        value_id: ValueId,
        vote_method: VoteMethod,
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
            ProposalKind::DeployContract { .. } => "deploy_contract",
            ProposalKind::ElectAccount { .. } => "elect_account",
            ProposalKind::FactoryInfoUpdate { .. } => "factory_info_update",
            ProposalKind::FunctionCall { .. } => "function_call",
            ProposalKind::RemoveGroup { .. } => "remove_group",
            ProposalKind::RemoveRule { .. } => "remove_rule",
            ProposalKind::RemoveValue { .. } => "remove_value",
            ProposalKind::Transfer { .. } => "transfer",
            ProposalKind::UnelectAccount { .. } => "unelect_account",
            ProposalKind::UpdateChoiceMetadata { .. } => "update_choice",
            ProposalKind::UpdateChoiceSize { .. } => "update_choice_size",
            ProposalKind::UpdateGroupName { .. } => "update_group_name",
            ProposalKind::UpdateGroupPermissions { .. } => "update_group_permissions",
            ProposalKind::UpdateGroupVoteMethod { .. } => "update_group_vote_method",
            ProposalKind::UpdateDefaultPolicy { .. } => "update_default_policy",
            ProposalKind::UpdateRule { .. } => "update_rule",
            ProposalKind::UpdateValueName { .. } => "update_value_name",
            ProposalKind::UpdateValue { .. } => "update_value",
            ProposalKind::UpdateValueVoteMethod { .. } => "update_value_vote_method",
            ProposalKind::UpgradeContract { .. } => "upgrade_contract",
            ProposalKind::UpgradeSelf { .. } => "upgrade_self",
            ProposalKind::Vote => "vote",
        }
    }
}