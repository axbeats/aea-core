use crate::*;

pub type ProposalKindString = String;

/// Kinds of proposals, doing different action.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalKind {
    AddAccountToGroup {
        group_id: GroupId,
        member_id: AccountId,
    },
    AddGroup {
        input: GroupInput,
    },
    AddValue {
        input: ValueInput,
    },
    FactoryInfoUpdate {
        factory_info: FactoryInfo,
    },
    FunctionCall {
        functions: Vec<FunctionCall>,
    },
    RemoveAccountFromGroup {
        group_id: GroupId,
        member_id: AccountId,
    },
    RemoveGroup {
        group_id: GroupId,
    },
    RemoveValue {
        value_id: ValueId,
    },
    /// Sets staking contract. Can only be proposed if staking contract is not set yet.
    SetStakingContract {
        staking_id: AccountId,
    },
    /// Transfers given amount of `token_id` from this DAO to `receiver_id`.
    /// If `msg` is not None, calls `ft_transfer_call` with given `msg`. Fails if this base token.
    /// For `ft_transfer` and `ft_transfer_call` `memo` is the `description` of the proposal.
    Transfer {
        input: TransferInput,
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
    UpdateValueName {
        value_id: ValueId,
        name: String,
    },
    UpdateValue {
        value_id: ValueId,
        data: ValueStructure,
    },
    UpdateValueVoteMethod {
        value_id: GroupId,
        vote_method: VoteMethod,
    },
    /// Upgrade another contract, by calling method with the code from given hash from blob store.
    UpgradeContract {
        receiver_id: AccountId,
        method_name: String,
        hash: Base58CryptoHash,
    },
    /// Upgrade this contract with given hash from blob store.
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
            ProposalKind::AddGroup { .. } => "add_group",
            ProposalKind::AddAccountToGroup { .. } => "add_member_to_group",
            ProposalKind::AddValue { .. } => "add_value",
            ProposalKind::FactoryInfoUpdate { .. } => "factory_info_update",
            ProposalKind::FunctionCall { .. } => "function_call",
            ProposalKind::RemoveGroup { .. } => "remove_group",
            ProposalKind::RemoveAccountFromGroup { .. } => "remove_member_from_group",
            ProposalKind::RemoveValue { .. } => "remove_value",
            ProposalKind::SetStakingContract { .. } => "set_staking_contract",
            ProposalKind::Transfer { .. } => "transfer",
            ProposalKind::UpdateChoiceMetadata { .. } => "update_choice",
            ProposalKind::UpdateChoiceSize { .. } => "update_choice_size",
            ProposalKind::UpdateGroupName { .. } => "update_group_name",
            ProposalKind::UpdateGroupPermissions { .. } => "update_group_permissions",
            ProposalKind::UpdateGroupVoteMethod { .. } => "update_group_vote_method",
            ProposalKind::UpdateDefaultPolicy { .. } => "update_default_policy",
            ProposalKind::UpdateValueName { .. } => "update_value_name",
            ProposalKind::UpdateValue { .. } => "update_value",
            ProposalKind::UpdateValueVoteMethod { .. } => "update_value_vote_method",
            ProposalKind::UpgradeContract { .. } => "upgrade_contract",
            ProposalKind::UpgradeSelf { .. } => "upgrade_self",
            ProposalKind::Vote => "vote",
        }
    }
}
