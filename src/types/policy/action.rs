use crate::*;

/// Function call arguments.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub contract_id: AccountId,
    pub method_name: String,
    pub args: Base64VecU8,
    pub deposit: U128,
    pub gas: U64,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ActionVoteChoice {
    pub governed_value_id: ValueId,
    pub voter_id: AccountId,
    pub role_id: RoleId,
    pub candidate: String,
    pub weight_kind: WeightKind,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Action {
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub role_id: RoleId,
    pub kind: ActionKind,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ActionInput {
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub role_id: RoleId,
    pub kind: ActionInputKind,
    pub location: Option<Point>,  // User's coordinates for region verification
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ActionInputKind {
    CreateProposal((ProposalInput, AttachedBond, DefaultBond)),
    VoteProposal((ProposalVoteInput, Policy)),
    VoteChoice(ChoiceVoteInput),
    VoteCalibration(CalibrationVoteInput),
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ActionKind {
    CreateProposal(ProposalInput),
    VoteProposal(ProposalVote, Policy, u128),
    VoteChoice(ChoiceVoteConfig),
    VoteCalibration(CalibrationVoteConfig),
}
