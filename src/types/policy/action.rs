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
    pub group_id: GroupId,
    pub candidate: String,
    pub weight_kind: WeightKind,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Action {
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    pub kind: ActionKind,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ActionInput {
    pub account_id: AccountId,
    pub dao_id: DaoId,
    pub group_id: GroupId,
    // pub default_policy: Policy,
    pub kind: ActionInputKind,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ActionInputKind {
    CreateProposal((ProposalInput, AttachedBond, DefaultBond)),
    VoteProposal((ProposalVoteInput, CurrentStage, Policy)), // CurrentStage is deprecated - always 1
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

// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub enum ActionVote {
//     Proposal(ActionVoteProposal),
//     Choice(ActionVoteChoice),
// }

// impl ActionVote {

//     pub fn voter_id(&self) -> &AccountId {
//         match self {
//             ActionVote::Proposal(proposal) => &proposal.voter_id,
//             ActionVote::Choice(choice) => &choice.voter_id,
//         }
//     }

//     pub fn group_id(&self) -> &GroupId {
//         match self {
//             ActionVote::Proposal(proposal) => &proposal.group_id,
//             ActionVote::Choice(choice) => &choice.group_id,
//         }
//     }
// }
